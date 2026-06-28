//! Drift monitoring for agent output quality over time.

/// Represents a single evaluation score sample.
#[derive(Debug, Clone)]
pub struct ScoreSample {
    pub timestamp_ms: u64,
    pub score: f64,
    pub label: Option<String>,
}

impl ScoreSample {
    pub fn new(timestamp_ms: u64, score: f64) -> Self {
        Self {
            timestamp_ms,
            score,
            label: None,
        }
    }
}

/// Detects statistical drift in a sequence of evaluation scores.
#[derive(Debug, Default)]
pub struct DriftMonitor {
    baseline_mean: Option<f64>,
    baseline_std: Option<f64>,
    threshold_sigma: f64,
}

impl DriftMonitor {
    /// Create a drift monitor with a given sigma threshold.
    pub fn new(threshold_sigma: f64) -> Self {
        Self {
            baseline_mean: None,
            baseline_std: None,
            threshold_sigma,
        }
    }

    /// Calibrate the baseline from a set of samples.
    pub fn calibrate(&mut self, samples: &[ScoreSample]) -> Result<(), DriftError> {
        if samples.is_empty() {
            return Err(DriftError::InsufficientData);
        }
        let n = samples.len() as f64;
        let mean = samples.iter().map(|s| s.score).sum::<f64>() / n;
        let variance = samples.iter().map(|s| (s.score - mean).powi(2)).sum::<f64>() / n;
        self.baseline_mean = Some(mean);
        self.baseline_std = Some(variance.sqrt());
        Ok(())
    }

    /// Check if a new score has drifted beyond the threshold.
    pub fn is_drifted(&self, score: f64) -> Result<bool, DriftError> {
        match (self.baseline_mean, self.baseline_std) {
            (Some(mean), Some(std)) if std > 0.0 => {
                let z = (score - mean).abs() / std;
                Ok(z > self.threshold_sigma)
            }
            (Some(_), Some(_)) => Ok(false),
            _ => Err(DriftError::NotCalibrated),
        }
    }
}

/// Errors produced by drift monitoring.
#[derive(Debug, PartialEq)]
pub enum DriftError {
    NotCalibrated,
    InsufficientData,
}

impl std::fmt::Display for DriftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriftError::NotCalibrated => write!(f, "drift monitor not calibrated"),
            DriftError::InsufficientData => write!(f, "insufficient data for calibration"),
        }
    }
}
