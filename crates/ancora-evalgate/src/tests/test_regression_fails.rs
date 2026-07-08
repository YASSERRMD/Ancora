use crate::regression::{detect, RegressionResult};
use crate::threshold::{MetricDirection, ThresholdKind, ThresholdPolicy};

#[test]
fn regression_below_threshold_blocks() {
    // accuracy drops from 0.90 to 0.80 - allowed drop is 2 pp
    let policy = ThresholdPolicy::new(
        "accuracy",
        MetricDirection::HigherIsBetter,
        ThresholdKind::Absolute(0.02),
    );
    let result = detect(0.90, 0.80, &policy);
    assert!(
        result.is_blocking(),
        "expected gate to block on 10 pp accuracy drop"
    );
    match result {
        RegressionResult::Regression { delta, threshold } => {
            assert!((delta - 0.10).abs() < 1e-9);
            assert!((threshold - 0.02).abs() < 1e-9);
        }
        other => panic!("unexpected result: {:?}", other),
    }
}

#[test]
fn regression_at_boundary_does_not_block() {
    // exactly at the threshold - should be within-threshold, not blocking
    let policy = ThresholdPolicy::new(
        "accuracy",
        MetricDirection::HigherIsBetter,
        ThresholdKind::Absolute(0.10),
    );
    let result = detect(0.90, 0.80, &policy);
    assert!(!result.is_blocking());
}
