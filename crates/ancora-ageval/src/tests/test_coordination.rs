use crate::coordination::CoordinationMetric;

#[test]
fn coordination_metric_on_fixture() {
    assert!((CoordinationMetric::score(5, 5) - 1.0).abs() < 1e-10);
}

#[test]
fn coordination_partial_success() {
    let score = CoordinationMetric::score(5, 3);
    assert!((score - 0.6).abs() < 1e-10);
}

#[test]
fn coordination_zero_assigned_returns_one() {
    assert_eq!(CoordinationMetric::score(0, 0), 1.0);
}
