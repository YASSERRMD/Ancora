//! A/B testing infrastructure for comparing agent variants.

use std::collections::HashMap;

/// Identifies an experiment variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantId(pub String);

/// A single A/B experiment definition.
#[derive(Debug, Clone)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub variants: Vec<VariantId>,
    /// Traffic split by variant; values should sum to 1.0.
    pub weights: HashMap<VariantId, f64>,
}

impl Experiment {
    /// Create an experiment with two equal-weight variants.
    pub fn two_way(id: impl Into<String>, name: impl Into<String>) -> Self {
        let control = VariantId("control".to_string());
        let treatment = VariantId("treatment".to_string());
        let mut weights = HashMap::new();
        weights.insert(control.clone(), 0.5);
        weights.insert(treatment.clone(), 0.5);
        Self {
            id: id.into(),
            name: name.into(),
            variants: vec![control, treatment],
            weights,
        }
    }

    /// Validate that weights sum to approximately 1.0.
    pub fn validate_weights(&self) -> bool {
        let total: f64 = self.weights.values().sum();
        (total - 1.0).abs() < 1e-9
    }
}

/// Accumulated metrics for a single variant in an experiment.
#[derive(Debug, Default, Clone)]
pub struct VariantMetrics {
    pub impressions: u64,
    pub total_score: f64,
}

impl VariantMetrics {
    pub fn record(&mut self, score: f64) {
        self.impressions += 1;
        self.total_score += score;
    }

    pub fn mean_score(&self) -> f64 {
        if self.impressions == 0 {
            0.0
        } else {
            self.total_score / self.impressions as f64
        }
    }
}

/// Collects per-variant metrics for an experiment.
#[derive(Debug, Default)]
pub struct ExperimentResults {
    pub metrics: HashMap<String, VariantMetrics>,
}

impl ExperimentResults {
    pub fn record(&mut self, variant: impl Into<String>, score: f64) {
        self.metrics
            .entry(variant.into())
            .or_default()
            .record(score);
    }

    /// Returns the variant with the highest mean score, if any.
    pub fn winner(&self) -> Option<&str> {
        self.metrics
            .iter()
            .max_by(|a, b| {
                a.1.mean_score()
                    .partial_cmp(&b.1.mean_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k.as_str())
    }
}
