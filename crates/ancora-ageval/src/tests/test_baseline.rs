use crate::baseline::{BaselineResult, BaselineStore};

#[test]
fn baseline_no_regression_when_equal() {
    let mut store = BaselineStore::new(0.0);
    store.set("planning", 0.9);
    let result = store.check("planning", 0.9);
    assert_eq!(result, BaselineResult::Passed { score: 0.9 });
}

#[test]
fn regression_detected_against_baseline() {
    let mut store = BaselineStore::new(0.0);
    store.set("planning", 0.9);
    let result = store.check("planning", 0.7);
    assert!(matches!(result, BaselineResult::Regressed { .. }));
    if let BaselineResult::Regressed { expected, actual, delta } = result {
        assert!((expected - 0.9).abs() < 1e-10);
        assert!((actual - 0.7).abs() < 1e-10);
        assert!((delta - (-0.2)).abs() < 1e-10);
    }
}

#[test]
fn baseline_no_prior_returned_for_unknown_metric() {
    let store = BaselineStore::new(0.05);
    assert_eq!(store.check("unknown", 0.8), BaselineResult::NoPrior);
}

#[test]
fn baseline_tolerance_prevents_false_regression() {
    let mut store = BaselineStore::new(0.05);
    store.set("routing", 0.9);
    // 0.86 is only 0.04 below 0.9, within tolerance 0.05
    let result = store.check("routing", 0.86);
    assert_eq!(result, BaselineResult::Passed { score: 0.86 });
}
