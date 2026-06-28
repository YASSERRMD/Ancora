/// Continuous evaluation module: tracks quality metrics over time.

use std::collections::VecDeque;

/// A time-series quality observation.
#[derive(Debug, Clone)]
pub struct QualityObservation {
    pub timestamp_ms: u64,
    pub metric: String,
    pub value: f64,
}

impl QualityObservation {
    pub fn new(timestamp_ms: u64, metric: impl Into<String>, value: f64) -> Self {
        Self {
            timestamp_ms,
            metric: metric.into(),
            value,
        }
    }
}

/// A rolling window tracker for a single metric.
#[derive(Debug)]
pub struct RollingTracker {
    pub metric: String,
    pub window_size: usize,
    observations: VecDeque<f64>,
}

impl RollingTracker {
    pub fn new(metric: impl Into<String>, window_size: usize) -> Self {
        Self {
            metric: metric.into(),
            window_size,
            observations: VecDeque::new(),
        }
    }

    pub fn record(&mut self, value: f64) {
        if self.observations.len() == self.window_size {
            self.observations.pop_front();
        }
        self.observations.push_back(value);
    }

    pub fn mean(&self) -> Option<f64> {
        if self.observations.is_empty() {
            return None;
        }
        let sum: f64 = self.observations.iter().sum();
        Some(sum / self.observations.len() as f64)
    }

    pub fn is_full(&self) -> bool {
        self.observations.len() == self.window_size
    }

    pub fn count(&self) -> usize {
        self.observations.len()
    }

    pub fn min(&self) -> Option<f64> {
        self.observations.iter().cloned().reduce(f64::min)
    }

    pub fn max(&self) -> Option<f64> {
        self.observations.iter().cloned().reduce(f64::max)
    }
}

/// Continuous eval engine that manages multiple metric trackers.
#[derive(Debug, Default)]
pub struct ContEvalEngine {
    trackers: std::collections::HashMap<String, RollingTracker>,
}

impl ContEvalEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_tracker(&mut self, tracker: RollingTracker) {
        self.trackers.insert(tracker.metric.clone(), tracker);
    }

    pub fn record(&mut self, metric: &str, value: f64) -> bool {
        if let Some(t) = self.trackers.get_mut(metric) {
            t.record(value);
            true
        } else {
            false
        }
    }

    pub fn mean(&self, metric: &str) -> Option<f64> {
        self.trackers.get(metric)?.mean()
    }

    pub fn tracker(&self, metric: &str) -> Option<&RollingTracker> {
        self.trackers.get(metric)
    }

    pub fn metrics(&self) -> Vec<&str> {
        self.trackers.keys().map(|s| s.as_str()).collect()
    }
}
