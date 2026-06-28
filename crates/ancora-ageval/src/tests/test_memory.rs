use crate::memory_metric::MemoryMetric;

#[test]
fn memory_metric_on_fixture() {
    assert!((MemoryMetric::score(10, 10) - 1.0).abs() < 1e-10);
}

#[test]
fn memory_partial_retention() {
    let score = MemoryMetric::score(3, 5);
    assert!((score - 0.6).abs() < 1e-10);
}

#[test]
fn memory_zero_total_returns_one() {
    assert_eq!(MemoryMetric::score(0, 0), 1.0);
}
