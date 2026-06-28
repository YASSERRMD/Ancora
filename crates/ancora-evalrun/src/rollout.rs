/// N-rollout per case - run the same case N times and collect all results.

use crate::executor::{CaseResult, EvalCase, Executor, Outcome};

/// A collection of N results for a single case.
#[derive(Debug, Clone)]
pub struct RolloutResult {
    pub case_id: String,
    pub results: Vec<CaseResult>,
}

impl RolloutResult {
    /// Count the number of passing rollouts.
    pub fn pass_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.outcome == Outcome::Pass)
            .count()
    }

    /// Count the number of failing rollouts.
    pub fn fail_count(&self) -> usize {
        self.results.len() - self.pass_count()
    }

    /// Pass rate as a fraction in [0.0, 1.0].
    pub fn pass_rate(&self) -> f64 {
        let n = self.results.len();
        if n == 0 {
            return 0.0;
        }
        self.pass_count() as f64 / n as f64
    }

    /// Mean latency across all rollouts in milliseconds.
    pub fn mean_latency_ms(&self) -> f64 {
        let n = self.results.len();
        if n == 0 {
            return 0.0;
        }
        self.results.iter().map(|r| r.latency_ms as f64).sum::<f64>() / n as f64
    }

    /// Total cost tokens across all rollouts.
    pub fn total_cost_tokens(&self) -> u64 {
        self.results.iter().map(|r| r.cost_tokens).sum()
    }
}

/// Rollout runner: runs each case N times.
pub struct RolloutRunner {
    pub n: usize,
}

impl RolloutRunner {
    pub fn new(n: usize) -> Self {
        assert!(n >= 1, "rollout count must be at least 1");
        Self { n }
    }

    /// Run a single case N times, using seed + rollout index for reproducibility.
    pub fn rollout<F>(
        &self,
        executor: &Executor,
        case: &EvalCase,
        infer: &F,
    ) -> RolloutResult
    where
        F: Fn(&str, u64) -> (String, u64, u64),
    {
        let base_seed = executor.config().seed;
        let results = (0..self.n)
            .map(|i| {
                // Vary the seed per rollout so each run is independent.
                let seed_i = base_seed.wrapping_add(i as u64);
                let (actual, latency_ms, cost_tokens) = infer(&case.input, seed_i);
                let passes =
                    (executor.config().scorer)(&actual, &case.expected);
                let outcome = if passes {
                    Outcome::Pass
                } else {
                    Outcome::Fail {
                        reason: format!(
                            "rollout {}: expected {:?} got {:?}",
                            i, case.expected, actual
                        ),
                    }
                };
                CaseResult {
                    case_id: case.id.clone(),
                    outcome,
                    actual,
                    latency_ms,
                    cost_tokens,
                }
            })
            .collect();
        RolloutResult {
            case_id: case.id.clone(),
            results,
        }
    }

    /// Run all cases N times.
    pub fn rollout_suite<F>(
        &self,
        executor: &Executor,
        cases: &[EvalCase],
        infer: &F,
    ) -> Vec<RolloutResult>
    where
        F: Fn(&str, u64) -> (String, u64, u64),
    {
        cases.iter().map(|c| self.rollout(executor, c, infer)).collect()
    }
}
