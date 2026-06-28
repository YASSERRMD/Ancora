/// Regression detection.
///
/// Compares a candidate metric value against a baseline and the threshold
/// policy to produce a typed `RegressionResult`.

use crate::threshold::{MetricDirection, ThresholdPolicy};

/// The outcome of a single metric comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionResult {
    /// The candidate is better than or equal to the baseline.
    Improvement { delta: f64 },
    /// The candidate is worse but within the allowed threshold.
    WithinThreshold { delta: f64 },
    /// The candidate exceeds the allowed regression.
    Regression { delta: f64, threshold: f64 },
}

impl RegressionResult {
    /// Returns `true` when the result blocks a gate.
    pub fn is_blocking(&self) -> bool {
        matches!(self, RegressionResult::Regression { .. })
    }
}

/// Check a single metric against its policy.
///
/// `baseline` - the stored baseline value.
/// `candidate` - the value from the current PR run.
/// `policy`   - the threshold policy for this metric.
pub fn detect(baseline: f64, candidate: f64, policy: &ThresholdPolicy) -> RegressionResult {
    let delta = match policy.direction {
        MetricDirection::HigherIsBetter => baseline - candidate,
        MetricDirection::LowerIsBetter => candidate - baseline,
    };

    if delta <= 0.0 {
        // No regression - candidate is at least as good as baseline.
        RegressionResult::Improvement { delta }
    } else {
        let threshold = policy.max_regression(baseline);
        if delta <= threshold {
            RegressionResult::WithinThreshold { delta }
        } else {
            RegressionResult::Regression { delta, threshold }
        }
    }
}
