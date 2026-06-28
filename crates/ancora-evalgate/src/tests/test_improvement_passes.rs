use crate::regression::{detect, RegressionResult};
use crate::threshold::{MetricDirection, ThresholdKind, ThresholdPolicy};

#[test]
fn improvement_does_not_block() {
    // accuracy improves: 0.85 -> 0.90 - gate must pass
    let policy = ThresholdPolicy::new(
        "accuracy",
        MetricDirection::HigherIsBetter,
        ThresholdKind::Absolute(0.02),
    );
    let result = detect(0.85, 0.90, &policy);
    assert!(!result.is_blocking());
    assert!(matches!(result, RegressionResult::Improvement { .. }));
}

#[test]
fn lower_is_better_improvement_passes() {
    // cost drops: 1.00 -> 0.80 - gate must pass
    let policy = ThresholdPolicy::new(
        "cost_usd",
        MetricDirection::LowerIsBetter,
        ThresholdKind::Relative(0.10),
    );
    let result = detect(1.00, 0.80, &policy);
    assert!(!result.is_blocking());
    assert!(matches!(result, RegressionResult::Improvement { .. }));
}
