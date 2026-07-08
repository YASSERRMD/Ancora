/// Drift detection module for detecting distribution shifts in inputs or outputs.

/// A simple numeric distribution summary.
#[derive(Debug, Clone)]
pub struct DistributionStats {
    pub mean: f64,
    pub std_dev: f64,
    pub count: usize,
}

impl DistributionStats {
    pub fn from_samples(samples: &[f64]) -> Option<Self> {
        if samples.is_empty() {
            return None;
        }
        let count = samples.len();
        let mean = samples.iter().sum::<f64>() / count as f64;
        let variance = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();
        Some(Self {
            mean,
            std_dev,
            count,
        })
    }
}

/// Drift test result.
#[derive(Debug, PartialEq)]
pub enum DriftStatus {
    /// No significant drift detected.
    NoDrift,
    /// Drift detected with a z-score magnitude.
    DriftDetected { z_score: f64 },
}

/// Detects drift between a baseline and a candidate distribution using mean z-score.
pub fn detect_drift(
    baseline: &DistributionStats,
    candidate_mean: f64,
    threshold: f64,
) -> DriftStatus {
    if baseline.std_dev == 0.0 {
        if (candidate_mean - baseline.mean).abs() < f64::EPSILON {
            return DriftStatus::NoDrift;
        }
        return DriftStatus::DriftDetected {
            z_score: f64::INFINITY,
        };
    }
    let z = (candidate_mean - baseline.mean).abs() / baseline.std_dev;
    if z > threshold {
        DriftStatus::DriftDetected { z_score: z }
    } else {
        DriftStatus::NoDrift
    }
}

/// Detect drift on a categorical feature via frequency shift.
#[derive(Debug)]
pub struct CategoryDriftDetector {
    pub baseline_freq: std::collections::HashMap<String, f64>,
    pub threshold: f64,
}

impl CategoryDriftDetector {
    pub fn new(baseline_freq: std::collections::HashMap<String, f64>, threshold: f64) -> Self {
        Self {
            baseline_freq,
            threshold,
        }
    }

    /// Returns the categories whose frequency shift exceeds the threshold.
    pub fn shifted_categories(
        &self,
        candidate_freq: &std::collections::HashMap<String, f64>,
    ) -> Vec<String> {
        let mut drifted = Vec::new();
        for (k, &base) in &self.baseline_freq {
            let cand = candidate_freq.get(k).copied().unwrap_or(0.0);
            if (cand - base).abs() > self.threshold {
                drifted.push(k.clone());
            }
        }
        drifted.sort();
        drifted
    }

    pub fn is_drifted(&self, candidate_freq: &std::collections::HashMap<String, f64>) -> bool {
        !self.shifted_categories(candidate_freq).is_empty()
    }
}
