//! Cost anomaly detection using Z-score based statistical analysis.

#[derive(Debug, Clone)]
pub struct AnomalyAlert {
    pub timestamp: u64,
    pub cost: f64,
    pub z_score: f64,
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct AnomalyDetector {
    /// Historical cost observations used to compute mean/stddev.
    history: Vec<f64>,
    /// Z-score threshold above which a value is flagged.
    threshold: f64,
}

impl AnomalyDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            history: Vec::new(),
            threshold,
        }
    }

    /// Add a historical observation (without checking for anomaly).
    pub fn add_observation(&mut self, cost: f64) {
        self.history.push(cost);
    }

    fn mean(&self) -> Option<f64> {
        if self.history.is_empty() {
            return None;
        }
        Some(self.history.iter().sum::<f64>() / self.history.len() as f64)
    }

    fn stddev(&self) -> Option<f64> {
        let mean = self.mean()?;
        if self.history.len() < 2 {
            return None;
        }
        let variance = self.history.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / (self.history.len() - 1) as f64;
        Some(variance.sqrt())
    }

    /// Check if a new cost value is anomalous. Returns an alert if so.
    pub fn check(&self, timestamp: u64, cost: f64) -> Option<AnomalyAlert> {
        let mean = self.mean()?;
        let std = self.stddev()?;
        if std == 0.0 {
            return None;
        }
        let z = (cost - mean) / std;
        if z.abs() > self.threshold {
            Some(AnomalyAlert {
                timestamp,
                cost,
                z_score: z,
                description: format!(
                    "Cost {:.4} deviates {:.2} std-devs from mean {:.4}",
                    cost, z, mean
                ),
            })
        } else {
            None
        }
    }

    /// Check and record the observation.
    pub fn observe(&mut self, timestamp: u64, cost: f64) -> Option<AnomalyAlert> {
        let alert = self.check(timestamp, cost);
        self.history.push(cost);
        alert
    }

    pub fn history(&self) -> &[f64] {
        &self.history
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anomaly_detected_on_spike() {
        let mut d = AnomalyDetector::new(2.0);
        for v in [1.0, 1.1, 0.9, 1.0, 1.05, 0.95] {
            d.add_observation(v);
        }
        let alert = d.check(99, 10.0);
        assert!(alert.is_some());
        assert!(alert.unwrap().z_score > 2.0);
    }

    #[test]
    fn normal_value_not_flagged() {
        let mut d = AnomalyDetector::new(2.0);
        for v in [1.0, 1.1, 0.9, 1.0, 1.05, 0.95] {
            d.add_observation(v);
        }
        let alert = d.check(99, 1.02);
        assert!(alert.is_none());
    }
}
