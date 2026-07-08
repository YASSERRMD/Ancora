use crate::regression::{detect, RegressionResult};
/// Cost regression gate.
///
/// Checks whether the per-run cost (in USD or token-equivalent units)
/// exceeds the baseline cost beyond the configured threshold.
use crate::threshold::{MetricDirection, ThresholdKind, ThresholdPolicy};

/// Configuration for the cost gate.
#[derive(Debug, Clone)]
pub struct CostGateConfig {
    /// Maximum allowed relative increase in cost (e.g. 0.10 = 10 %).
    pub max_relative_increase: f64,
}

impl Default for CostGateConfig {
    fn default() -> Self {
        Self {
            max_relative_increase: 0.10,
        }
    }
}

/// Evaluate whether `candidate_cost` regresses beyond `baseline_cost`.
///
/// Returns the `RegressionResult` for the cost metric.
pub fn check_cost(
    baseline_cost: f64,
    candidate_cost: f64,
    config: &CostGateConfig,
) -> RegressionResult {
    let policy = ThresholdPolicy::new(
        "cost_usd",
        MetricDirection::LowerIsBetter,
        ThresholdKind::Relative(config.max_relative_increase),
    );
    detect(baseline_cost, candidate_cost, &policy)
}

/// Convenience wrapper: returns `true` when the cost gate should block.
pub fn blocks(baseline_cost: f64, candidate_cost: f64, config: &CostGateConfig) -> bool {
    check_cost(baseline_cost, candidate_cost, config).is_blocking()
}
