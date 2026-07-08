//! Rolling quality metrics over a sliding time window.
//!
//! Keeps a fixed-size ring buffer of quality scores and computes
//! mean, min, max, and standard deviation over the window.

/// A single quality observation with a timestamp (epoch seconds).
#[derive(Debug, Clone)]
pub struct QualityObservation {
    pub timestamp: u64,
    pub score: f64,
}

/// A rolling window of quality scores.
#[derive(Debug)]
pub struct RollingMetric {
    capacity: usize,
    observations: Vec<QualityObservation>,
}

impl RollingMetric {
    /// Create a new rolling metric with the given window capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        RollingMetric {
            capacity,
            observations: Vec::with_capacity(capacity),
        }
    }

    /// Push a new observation. Oldest entry is evicted when capacity is full.
    pub fn push(&mut self, timestamp: u64, score: f64) {
        if self.observations.len() == self.capacity {
            self.observations.remove(0);
        }
        self.observations
            .push(QualityObservation { timestamp, score });
    }

    /// Number of observations currently stored.
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns true if there are no observations.
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Compute the mean score. Returns `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.observations.is_empty() {
            return None;
        }
        let sum: f64 = self.observations.iter().map(|o| o.score).sum();
        Some(sum / self.observations.len() as f64)
    }

    /// Compute the minimum score. Returns `None` if empty.
    pub fn min(&self) -> Option<f64> {
        self.observations.iter().map(|o| o.score).reduce(f64::min)
    }

    /// Compute the maximum score. Returns `None` if empty.
    pub fn max(&self) -> Option<f64> {
        self.observations.iter().map(|o| o.score).reduce(f64::max)
    }

    /// Compute the population standard deviation. Returns `None` if fewer
    /// than two observations.
    pub fn std_dev(&self) -> Option<f64> {
        if self.observations.len() < 2 {
            return None;
        }
        let mean = self.mean()?;
        let variance: f64 = self
            .observations
            .iter()
            .map(|o| {
                let diff = o.score - mean;
                diff * diff
            })
            .sum::<f64>()
            / self.observations.len() as f64;
        Some(variance.sqrt())
    }

    /// Return all observations in chronological order.
    pub fn observations(&self) -> &[QualityObservation] {
        &self.observations
    }

    /// Return the most recent observation, if any.
    pub fn latest(&self) -> Option<&QualityObservation> {
        self.observations.last()
    }
}
