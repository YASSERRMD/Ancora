//! Regression gates: block deploys when eval scores regress past a threshold.

use crate::evals_platform::EvalRunSummary;

/// Configuration for a regression gate.
#[derive(Debug, Clone)]
pub struct RegressionGate {
    pub name: String,
    /// Minimum acceptable pass rate (0.0 - 1.0).
    pub min_pass_rate: f64,
    /// Minimum acceptable mean score.
    pub min_mean_score: f64,
}

impl RegressionGate {
    pub fn new(name: impl Into<String>, min_pass_rate: f64, min_mean_score: f64) -> Self {
        Self {
            name: name.into(),
            min_pass_rate,
            min_mean_score,
        }
    }
}

/// The verdict returned by a gate check.
#[derive(Debug, Clone, PartialEq)]
pub enum GateVerdict {
    Pass,
    Fail(GateFailReason),
}

/// Reason a regression gate failed.
#[derive(Debug, Clone, PartialEq)]
pub enum GateFailReason {
    PassRateBelow { actual: f64, required: f64 },
    MeanScoreBelow { actual: f64, required: f64 },
}

impl RegressionGate {
    /// Evaluate a gate against an eval run summary.
    pub fn evaluate(&self, summary: &EvalRunSummary) -> GateVerdict {
        let pass_rate = summary.pass_rate();
        if pass_rate < self.min_pass_rate {
            return GateVerdict::Fail(GateFailReason::PassRateBelow {
                actual: pass_rate,
                required: self.min_pass_rate,
            });
        }
        let mean_score = summary.mean_score();
        if mean_score < self.min_mean_score {
            return GateVerdict::Fail(GateFailReason::MeanScoreBelow {
                actual: mean_score,
                required: self.min_mean_score,
            });
        }
        GateVerdict::Pass
    }
}

/// Runs all gates and returns the first failure, if any.
pub fn run_gates(gates: &[RegressionGate], summary: &EvalRunSummary) -> Option<GateFailReason> {
    for gate in gates {
        if let GateVerdict::Fail(reason) = gate.evaluate(summary) {
            return Some(reason);
        }
    }
    None
}
