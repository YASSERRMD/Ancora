/// Latency regression gate.
///
/// Checks whether the p50/p95/p99 latency (in milliseconds) for a PR run
/// regresses beyond the baseline latency by more than the allowed threshold.

use crate::threshold::{MetricDirection, ThresholdKind, ThresholdPolicy};
use crate::regression::{detect, RegressionResult};

/// Which latency percentile to evaluate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Percentile {
    P50,
    P95,
    P99,
}

impl Percentile {
    fn metric_name(&self) -> &'static str {
        match self {
            Percentile::P50 => "latency_p50_ms",
            Percentile::P95 => "latency_p95_ms",
            Percentile::P99 => "latency_p99_ms",
        }
    }
}

/// Configuration for the latency gate.
#[derive(Debug, Clone)]
pub struct LatencyGateConfig {
    /// Maximum allowed relative increase in latency (e.g. 0.20 = 20 %).
    pub max_relative_increase: f64,
    /// Which percentile(s) to enforce.
    pub percentiles: Vec<Percentile>,
}

impl Default for LatencyGateConfig {
    fn default() -> Self {
        Self {
            max_relative_increase: 0.20,
            percentiles: vec![Percentile::P50, Percentile::P95],
        }
    }
}

/// Result for a single latency percentile check.
#[derive(Debug, Clone)]
pub struct LatencyCheckResult {
    pub percentile: Percentile,
    pub regression: RegressionResult,
}

impl LatencyCheckResult {
    pub fn is_blocking(&self) -> bool {
        self.regression.is_blocking()
    }
}

/// Evaluate latency for all configured percentiles.
///
/// `baseline_ms` and `candidate_ms` are indexed in the same order as
/// `config.percentiles`.
pub fn check_latency(
    baseline_ms: &[f64],
    candidate_ms: &[f64],
    config: &LatencyGateConfig,
) -> Vec<LatencyCheckResult> {
    config
        .percentiles
        .iter()
        .zip(baseline_ms.iter().zip(candidate_ms.iter()))
        .map(|(&pct, (&base, &cand))| {
            let policy = ThresholdPolicy::new(
                pct.metric_name(),
                MetricDirection::LowerIsBetter,
                ThresholdKind::Relative(config.max_relative_increase),
            );
            LatencyCheckResult {
                percentile: pct,
                regression: detect(base, cand, &policy),
            }
        })
        .collect()
}

/// Convenience wrapper: returns `true` when any configured percentile blocks.
pub fn any_blocks(
    baseline_ms: &[f64],
    candidate_ms: &[f64],
    config: &LatencyGateConfig,
) -> bool {
    check_latency(baseline_ms, candidate_ms, config)
        .iter()
        .any(|r| r.is_blocking())
}
