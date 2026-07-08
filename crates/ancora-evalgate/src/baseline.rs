/// Baseline storage per dataset.
///
/// A `Baseline` records the accepted metric values for a named dataset.
/// Values are keyed by metric name and stored as `f64`.
use std::collections::HashMap;

/// A snapshot of baseline metric values for one dataset.
#[derive(Debug, Clone)]
pub struct Baseline {
    pub dataset: String,
    pub metrics: HashMap<String, f64>,
}

impl Baseline {
    /// Create an empty baseline for the given dataset.
    pub fn new(dataset: impl Into<String>) -> Self {
        Self {
            dataset: dataset.into(),
            metrics: HashMap::new(),
        }
    }

    /// Record (or overwrite) a metric value in this baseline.
    pub fn set(&mut self, metric: impl Into<String>, value: f64) {
        self.metrics.insert(metric.into(), value);
    }

    /// Retrieve a baseline value, returning `None` when no baseline exists yet.
    pub fn get(&self, metric: &str) -> Option<f64> {
        self.metrics.get(metric).copied()
    }

    /// Update this baseline with the values from a candidate run, keeping
    /// all other previously recorded metrics intact.
    pub fn update_from(&mut self, other: &HashMap<String, f64>) {
        for (k, v) in other {
            self.metrics.insert(k.clone(), *v);
        }
    }
}

/// In-memory store that maps dataset names to their baseline.
#[derive(Debug, Default)]
pub struct BaselineStore {
    entries: HashMap<String, Baseline>,
}

impl BaselineStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace the baseline for a dataset.
    pub fn upsert(&mut self, baseline: Baseline) {
        self.entries.insert(baseline.dataset.clone(), baseline);
    }

    /// Retrieve an immutable reference to a baseline, if it exists.
    pub fn get(&self, dataset: &str) -> Option<&Baseline> {
        self.entries.get(dataset)
    }

    /// Retrieve a mutable reference to a baseline, if it exists.
    pub fn get_mut(&mut self, dataset: &str) -> Option<&mut Baseline> {
        self.entries.get_mut(dataset)
    }
}
