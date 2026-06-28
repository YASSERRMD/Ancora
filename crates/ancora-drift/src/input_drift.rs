//! Input drift detection.
//!
//! Compares the statistical distribution of incoming request inputs against a
//! reference distribution and reports whether significant drift has occurred.

use crate::reference::{ReferenceDistribution, Stats};

/// Result of an input drift check.
#[derive(Debug, Clone, PartialEq)]
pub struct InputDriftResult {
    /// Whether drift was detected.
    pub drifted: bool,
    /// Z-score of the mean input length shift.
    pub mean_z_score: f64,
    /// Absolute shift in mean input length (chars).
    pub mean_shift: f64,
    /// The threshold used for detection.
    pub threshold_z: f64,
}

/// Detector for changes in the distribution of model inputs.
#[derive(Debug, Clone)]
pub struct InputDriftDetector {
    /// Number of standard deviations away from reference mean to flag as drift.
    pub threshold_z: f64,
}

impl Default for InputDriftDetector {
    fn default() -> Self {
        Self { threshold_z: 3.0 }
    }
}

impl InputDriftDetector {
    pub fn new(threshold_z: f64) -> Self {
        Self { threshold_z }
    }

    /// Check whether `current` has drifted from `reference`.
    ///
    /// Returns `Err` if the reference standard deviation is zero (degenerate
    /// distribution) or the current stats are missing.
    pub fn check(
        &self,
        reference: &ReferenceDistribution,
        current: &Stats,
    ) -> Result<InputDriftResult, String> {
        let ref_std = reference.input_len.std_dev();
        if ref_std == 0.0 {
            return Err("Reference input_len std_dev is zero; cannot compute z-score".into());
        }
        let mean_shift = current.mean - reference.input_len.mean;
        let mean_z_score = mean_shift / ref_std;
        let drifted = mean_z_score.abs() > self.threshold_z;
        Ok(InputDriftResult {
            drifted,
            mean_z_score,
            mean_shift,
            threshold_z: self.threshold_z,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::{ReferenceBuilder, Stats};

    fn make_ref() -> ReferenceDistribution {
        let mut b = ReferenceBuilder::new();
        for _ in 0..100 {
            b.add("hello world", "ok", 100, 50, &[], "openai");
        }
        b.build().unwrap()
    }

    #[test]
    fn no_drift_on_identical_inputs() {
        let reference = make_ref();
        // Mean input length = 11 chars; std_dev near 0 for identical strings.
        // Use a reference with variance.
        let mut b = ReferenceBuilder::new();
        b.add("hello", "ok", 100, 50, &[], "openai");
        b.add("hello world extra", "ok", 100, 50, &[], "openai");
        let reference = b.build().unwrap();

        let current = Stats::from_slice(&[11.0]).unwrap();
        let detector = InputDriftDetector::new(3.0);
        let result = detector.check(&reference, &current).unwrap();
        assert!(!result.drifted);
    }

    #[test]
    fn drift_detected_on_large_shift() {
        let mut b = ReferenceBuilder::new();
        for _ in 0..50 {
            b.add("hi", "ok", 100, 50, &[], "openai");
        }
        for _ in 0..50 {
            b.add("hello", "ok", 100, 50, &[], "openai");
        }
        let reference = b.build().unwrap();
        // Current mean is 1000 chars - very far from reference mean ~3.5
        let current = Stats::from_slice(&[1000.0]).unwrap();
        let detector = InputDriftDetector::new(3.0);
        let result = detector.check(&reference, &current).unwrap();
        assert!(result.drifted);
        assert!(result.mean_z_score > 3.0);
    }
}
