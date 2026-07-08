/// Flaky-eval handling.
///
/// Some evals are inherently noisy; instead of failing immediately on a
/// single bad run, the gate retries up to a configurable limit and only
/// blocks when the majority of retried runs show regression.
use crate::regression::RegressionResult;

/// Policy that governs retry behaviour for a metric.
#[derive(Debug, Clone)]
pub struct FlakyPolicy {
    /// Maximum number of retry attempts (total runs = 1 + max_retries).
    pub max_retries: usize,
    /// Fraction of runs that must show regression to block the gate.
    /// E.g. 0.5 means more than half must regress.
    pub block_fraction: f64,
}

impl FlakyPolicy {
    pub fn new(max_retries: usize, block_fraction: f64) -> Self {
        Self {
            max_retries,
            block_fraction,
        }
    }
}

/// Aggregate the results of multiple runs of the same metric.
///
/// Returns `true` when the gate should block (enough runs regressed).
pub fn should_block(results: &[RegressionResult], policy: &FlakyPolicy) -> bool {
    if results.is_empty() {
        return false;
    }
    let blocking = results.iter().filter(|r| r.is_blocking()).count();
    let fraction = blocking as f64 / results.len() as f64;
    fraction > policy.block_fraction
}

/// Represents the outcome after flaky-retry evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlakyOutcome {
    /// Not enough runs regressed - gate passes.
    Passed { retries: usize },
    /// Majority of runs showed regression - gate fails.
    Failed {
        retries: usize,
        blocking_runs: usize,
    },
}

/// Run the flaky-eval retry loop.
///
/// `run_fn` is called once per attempt and returns the `RegressionResult` for
/// that attempt.  The closure receives the attempt index (0-based).
pub fn evaluate_with_retry<F>(policy: &FlakyPolicy, mut run_fn: F) -> FlakyOutcome
where
    F: FnMut(usize) -> RegressionResult,
{
    let total = 1 + policy.max_retries;
    let mut results = Vec::with_capacity(total);

    for i in 0..total {
        results.push(run_fn(i));
    }

    let blocking = results.iter().filter(|r| r.is_blocking()).count();
    if should_block(&results, policy) {
        FlakyOutcome::Failed {
            retries: policy.max_retries,
            blocking_runs: blocking,
        }
    } else {
        FlakyOutcome::Passed {
            retries: policy.max_retries,
        }
    }
}
