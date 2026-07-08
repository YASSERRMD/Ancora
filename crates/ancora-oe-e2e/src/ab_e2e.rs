/// A/B experiment module for running and concluding controlled experiments.
use std::collections::HashMap;

/// A variant in an A/B experiment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub id: String,
    pub description: String,
}

impl Variant {
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
        }
    }
}

/// Outcome metric values collected per variant.
#[derive(Debug, Default)]
pub struct ExperimentMetrics {
    data: HashMap<String, Vec<f64>>,
}

impl ExperimentMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, variant_id: &str, value: f64) {
        self.data
            .entry(variant_id.to_string())
            .or_default()
            .push(value);
    }

    pub fn mean(&self, variant_id: &str) -> Option<f64> {
        let values = self.data.get(variant_id)?;
        if values.is_empty() {
            return None;
        }
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }

    pub fn count(&self, variant_id: &str) -> usize {
        self.data.get(variant_id).map(|v| v.len()).unwrap_or(0)
    }
}

/// Conclusion of an A/B experiment.
#[derive(Debug, PartialEq)]
pub enum ExperimentConclusion {
    Winner(String),
    Tie,
    Inconclusive,
}

/// Concludes an A/B experiment given metrics and a minimum sample size.
pub fn conclude_experiment(
    metrics: &ExperimentMetrics,
    variants: &[Variant],
    min_samples: usize,
) -> ExperimentConclusion {
    if variants.len() < 2 {
        return ExperimentConclusion::Inconclusive;
    }

    // Check all have enough samples.
    for v in variants {
        if metrics.count(&v.id) < min_samples {
            return ExperimentConclusion::Inconclusive;
        }
    }

    // Find the variant with the highest mean.
    let mut best_id: Option<String> = None;
    let mut best_mean = f64::NEG_INFINITY;
    let mut tie = false;

    for v in variants {
        if let Some(m) = metrics.mean(&v.id) {
            if (m - best_mean).abs() < f64::EPSILON {
                tie = true;
            } else if m > best_mean {
                best_mean = m;
                best_id = Some(v.id.clone());
                tie = false;
            }
        }
    }

    if tie {
        ExperimentConclusion::Tie
    } else if let Some(id) = best_id {
        ExperimentConclusion::Winner(id)
    } else {
        ExperimentConclusion::Inconclusive
    }
}
