/// Per-provider quality tracking.
///
/// Aggregates quality scores across all models served by each provider,
/// enabling cross-provider comparison and alerting.
use crate::rolling_metric::RollingMetric;
use std::collections::HashMap;

/// Quality record for a single provider.
#[derive(Debug)]
pub struct ProviderQuality {
    pub provider: String,
    metric: RollingMetric,
    model_scores: HashMap<String, Vec<f64>>,
}

impl ProviderQuality {
    /// Create a new provider quality tracker.
    pub fn new(provider: impl Into<String>, capacity: usize) -> Self {
        ProviderQuality {
            provider: provider.into(),
            metric: RollingMetric::new(capacity),
            model_scores: HashMap::new(),
        }
    }

    /// Record a quality score for a specific model under this provider.
    pub fn record(&mut self, model: &str, timestamp: u64, score: f64) {
        self.metric.push(timestamp, score);
        self.model_scores
            .entry(model.to_string())
            .or_default()
            .push(score);
    }

    /// Aggregate mean across all models under this provider.
    pub fn mean(&self) -> Option<f64> {
        self.metric.mean()
    }

    /// Mean score for a specific model under this provider.
    pub fn model_mean(&self, model: &str) -> Option<f64> {
        let scores = self.model_scores.get(model)?;
        if scores.is_empty() {
            return None;
        }
        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }

    /// List of models tracked under this provider.
    pub fn tracked_models(&self) -> Vec<&str> {
        self.model_scores.keys().map(|k| k.as_str()).collect()
    }

    /// Total number of observations recorded.
    pub fn observation_count(&self) -> usize {
        self.metric.len()
    }

    /// Raw rolling metric access.
    pub fn metric(&self) -> &RollingMetric {
        &self.metric
    }
}

/// Registry that tracks quality for multiple providers.
#[derive(Debug, Default)]
pub struct ProviderTracker {
    window_capacity: usize,
    providers: HashMap<String, ProviderQuality>,
}

impl ProviderTracker {
    /// Create a new provider tracker with the given window capacity.
    pub fn new(window_capacity: usize) -> Self {
        ProviderTracker {
            window_capacity,
            providers: HashMap::new(),
        }
    }

    /// Record a score for a model under a provider.
    pub fn record(&mut self, provider: &str, model: &str, timestamp: u64, score: f64) {
        let cap = self.window_capacity;
        self.providers
            .entry(provider.to_string())
            .or_insert_with(|| ProviderQuality::new(provider, cap))
            .record(model, timestamp, score);
    }

    /// Get quality data for a provider.
    pub fn get(&self, provider: &str) -> Option<&ProviderQuality> {
        self.providers.get(provider)
    }

    /// Return all tracked provider names.
    pub fn providers(&self) -> Vec<&str> {
        self.providers.keys().map(|k| k.as_str()).collect()
    }

    /// Number of providers tracked.
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }
}
