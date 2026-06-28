use crate::cost_gate::{blocks, check_cost, CostGateConfig};
use crate::regression::RegressionResult;

#[test]
fn cost_regression_beyond_threshold_blocks() {
    // baseline $1.00, candidate $1.20 - 20% increase, threshold 10%
    let config = CostGateConfig { max_relative_increase: 0.10 };
    assert!(blocks(1.00, 1.20, &config), "20% cost increase should block");
}

#[test]
fn cost_regression_within_threshold_does_not_block() {
    // baseline $1.00, candidate $1.05 - 5% increase, threshold 10%
    let config = CostGateConfig { max_relative_increase: 0.10 };
    assert!(!blocks(1.00, 1.05, &config), "5% cost increase should not block");
}

#[test]
fn cost_improvement_returns_improvement_result() {
    let config = CostGateConfig::default();
    let result = check_cost(1.00, 0.80, &config);
    assert!(matches!(result, RegressionResult::Improvement { .. }));
}
