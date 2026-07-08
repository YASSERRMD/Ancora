use crate::latency_gate::{any_blocks, check_latency, LatencyGateConfig, Percentile};
use crate::regression::RegressionResult;

#[test]
fn latency_regression_beyond_threshold_blocks() {
    // p50 baseline 100ms, candidate 130ms - 30% increase, threshold 20%
    let config = LatencyGateConfig {
        max_relative_increase: 0.20,
        percentiles: vec![Percentile::P50],
    };
    assert!(any_blocks(&[100.0], &[130.0], &config));
}

#[test]
fn latency_improvement_does_not_block() {
    let config = LatencyGateConfig {
        max_relative_increase: 0.20,
        percentiles: vec![Percentile::P50, Percentile::P95],
    };
    // both percentiles improve
    assert!(!any_blocks(&[100.0, 200.0], &[90.0, 180.0], &config));
}

#[test]
fn latency_within_threshold_does_not_block() {
    let config = LatencyGateConfig {
        max_relative_increase: 0.20,
        percentiles: vec![Percentile::P95],
    };
    // 10% increase, within 20% threshold
    let results = check_latency(&[200.0], &[220.0], &config);
    assert_eq!(results.len(), 1);
    assert!(!results[0].is_blocking());
    assert!(matches!(
        results[0].regression,
        RegressionResult::WithinThreshold { .. }
    ));
}
