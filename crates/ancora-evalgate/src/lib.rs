//! ancora-evalgate - CI evaluation gates for quality, cost, and latency regressions.
//!
//! This crate provides significance-aware gates that block CI when a PR
//! causes a statistically significant regression in eval metrics.

pub mod baseline;
pub mod cost_gate;
pub mod flaky;
pub mod gate;
pub mod latency_gate;
pub mod pr_eval;
pub mod regression;
pub mod report;
pub mod significance;
pub mod threshold;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod test_baseline_updates;
    mod test_cost_regression;
    mod test_flaky_retried;
    mod test_gate_report;
    mod test_improvement_passes;
    mod test_insignificant_no_fail;
    mod test_latency_regression;
    mod test_pr_eval;
    mod test_regression_fails;
}
