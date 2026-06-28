//! Output drift detection.
//!
//! Compares the statistical properties of model outputs against a reference
//! distribution to detect changes in response style, length, or quality proxy.

use crate::reference::{ReferenceDistribution, Stats};

/// Result of an output drift check.
#[derive(Debug, Clone, PartialEq)]
pub struct OutputDriftResult {
    /// Whether output drift was detected.
    pub drifted: bool,
    /// Z-score of the mean output length shift.
    pub mean_z_score: f64,
    /// Absolute shift in mean output length (chars).
    pub mean_shift: f64,
    /// The threshold used.
    pub threshold_z: f64,
}

/// Detector for changes in model output distributions.
#[derive(Debug, Clone)]
pub struct OutputDriftDetector {
    pub threshold_z: f64,
}

impl Default for OutputDriftDetector {
    fn default() -> Self {
        Self { threshold_z: 3.0 }
    }
}

impl OutputDriftDetector {
    pub fn new(threshold_z: f64) -> Self {
        Self { threshold_z }
    }

    /// Compare `current` output stats against the `reference` distribution.
    pub fn check(
        &self,
        reference: &ReferenceDistribution,
        current: &Stats,
    ) -> Result<OutputDriftResult, String> {
        let ref_std = reference.output_len.std_dev();
        if ref_std == 0.0 {
            return Err("Reference output_len std_dev is zero".into());
        }
        let mean_shift = current.mean - reference.output_len.mean;
        let mean_z_score = mean_shift / ref_std;
        let drifted = mean_z_score.abs() > self.threshold_z;
        Ok(OutputDriftResult {
            drifted,
            mean_z_score,
            mean_shift,
            threshold_z: self.threshold_z,
        })
    }
}

/// Computes a simple vocabulary richness proxy: unique character bigrams / total bigrams.
pub fn bigram_richness(text: &str) -> f64 {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() < 2 {
        return 0.0;
    }
    let total = chars.len() - 1;
    let unique: std::collections::HashSet<(char, char)> =
        chars.windows(2).map(|w| (w[0], w[1])).collect();
    unique.len() as f64 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::ReferenceBuilder;

    #[test]
    fn no_drift_on_stable_outputs() {
        let mut b = ReferenceBuilder::new();
        b.add("q", "short answer", 100, 50, &[], "openai");
        b.add("q", "brief reply", 100, 50, &[], "openai");
        let reference = b.build().unwrap();

        // Current output is similar in length to reference mean
        let current = Stats::from_slice(&[11.0]).unwrap();
        let detector = OutputDriftDetector::new(3.0);
        // reference std_dev should be non-zero since lengths differ
        let result = detector.check(&reference, &current).unwrap();
        assert!(!result.drifted);
    }

    #[test]
    fn drift_detected_on_very_long_output() {
        let mut b = ReferenceBuilder::new();
        b.add("q", "ok", 100, 50, &[], "openai");
        b.add("q", "yes", 100, 50, &[], "openai");
        let reference = b.build().unwrap();
        // current mean = 5000 chars vs reference mean of ~2.5
        let current = Stats::from_slice(&[5000.0]).unwrap();
        let detector = OutputDriftDetector::new(3.0);
        let result = detector.check(&reference, &current).unwrap();
        assert!(result.drifted);
    }

    #[test]
    fn bigram_richness_range() {
        let richness = bigram_richness("abcde");
        assert!(richness > 0.0 && richness <= 1.0);
        // "aaaa" has 3 bigrams all identical ("aa"), so richness = 1/3.
        let r = bigram_richness("aaaa");
        assert!((r - 1.0 / 3.0).abs() < 1e-9, "expected 1/3, got {r}");
    }
}
