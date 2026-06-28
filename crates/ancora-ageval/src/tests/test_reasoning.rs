use crate::reasoning_metric::ReasoningMetric;

#[test]
fn reasoning_metric_on_fixture() {
    assert!((ReasoningMetric::score(3, 3) - 1.0).abs() < 1e-10);
}

#[test]
fn reasoning_partial_verified() {
    let score = ReasoningMetric::score(2, 3);
    assert!((score - 2.0 / 3.0).abs() < 1e-10);
}

#[test]
fn reasoning_zero_total_returns_one() {
    assert_eq!(ReasoningMetric::score(0, 0), 1.0);
}
