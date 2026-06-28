/// Per-model quality tracking.
///
/// Maintains a rolling quality history for each model that passes
/// through the evaluation pipeline.

use crate::rolling_metric::RollingMetric;
use std::collections::HashMap;

/// Quality record for a single model.
#[derive(Debug)]
pub struct ModelQuality {
    pub model: String,
    metric: RollingMetric,
}

impl ModelQuality {
    /// Create a new model quality tracker with the given window capacity.
    pub fn new(model: impl Into<String>, capacity: usize) -> Self {
        ModelQuality {
            model: model.into(),
            metric: RollingMetric::new(capacity),
        }
    }

    /// Record a quality score at the given timestamp.
    pub fn record(&mut self, timestamp: u64, score: f64) {
        self.metric.push(timestamp, score);
    }

    /// Current mean quality score.
    pub fn mean(&self) -> Option<f64> {
        self.metric.mean()
    }

    /// Latest quality score observed.
    pub fn latest_score(&self) -> Option<f64> {
        self.metric.latest().map(|o| o.score)
    }

    /// Number of recorded observations.
    pub fn observation_count(&self) -> usize {
        self.metric.len()
    }

    /// Raw metric access for advanced analytics.
    pub fn metric(&self) -> &RollingMetric {
        &self.metric
    }
}

/// Registry that tracks quality for multiple models.
#[derive(Debug, Default)]
pub struct ModelTracker {
    window_capacity: usize,
    models: HashMap<String, ModelQuality>,
}

impl ModelTracker {
    /// Create a tracker with the given rolling window capacity per model.
    pub fn new(window_capacity: usize) -> Self {
        ModelTracker {
            window_capacity,
            models: HashMap::new(),
        }
    }

    /// Record a score for a model. Creates a new entry if not seen before.
    pub fn record(&mut self, model: &str, timestamp: u64, score: f64) {
        let cap = self.window_capacity;
        self.models
            .entry(model.to_string())
            .or_insert_with(|| ModelQuality::new(model, cap))
            .record(timestamp, score);
    }

    /// Get the quality entry for a model, if it exists.
    pub fn get(&self, model: &str) -> Option<&ModelQuality> {
        self.models.get(model)
    }

    /// Return all tracked model names.
    pub fn models(&self) -> Vec<&str> {
        self.models.keys().map(|k| k.as_str()).collect()
    }

    /// Return the number of models being tracked.
    pub fn model_count(&self) -> usize {
        self.models.len()
    }
}
