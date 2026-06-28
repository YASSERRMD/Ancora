/// Eval run executor - drives evaluation cases through the pipeline.

use std::collections::HashMap;

/// Unique identifier for an evaluation run.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId(pub String);

/// A single eval case with an input and expected output.
#[derive(Debug, Clone)]
pub struct EvalCase {
    pub id: String,
    pub input: String,
    pub expected: String,
}

/// Outcome of running a single eval case.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Pass,
    Fail { reason: String },
}

/// Result of running a single eval case including metadata.
#[derive(Debug, Clone)]
pub struct CaseResult {
    pub case_id: String,
    pub outcome: Outcome,
    pub actual: String,
    pub latency_ms: u64,
    pub cost_tokens: u64,
}

/// A scorer function type that compares actual vs expected output.
pub type ScorerFn = fn(&str, &str) -> bool;

/// Configuration for an eval run.
#[derive(Debug, Clone)]
pub struct RunConfig {
    pub run_id: RunId,
    pub scorer: ScorerFn,
    pub seed: u64,
}

/// The executor that runs eval cases.
pub struct Executor {
    config: RunConfig,
}

impl Executor {
    pub fn new(config: RunConfig) -> Self {
        Self { config }
    }

    /// Execute a single eval case using a provided inference function.
    pub fn run_case<F>(&self, case: &EvalCase, infer: &F) -> CaseResult
    where
        F: Fn(&str, u64) -> (String, u64, u64),
    {
        let (actual, latency_ms, cost_tokens) =
            infer(&case.input, self.config.seed);
        let passes = (self.config.scorer)(&actual, &case.expected);
        let outcome = if passes {
            Outcome::Pass
        } else {
            Outcome::Fail {
                reason: format!(
                    "expected {:?} got {:?}",
                    case.expected, actual
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
    }

    /// Execute all cases in the suite.
    pub fn run_suite<F>(
        &self,
        cases: &[EvalCase],
        infer: &F,
    ) -> Vec<CaseResult>
    where
        F: Fn(&str, u64) -> (String, u64, u64),
    {
        cases.iter().map(|c| self.run_case(c, infer)).collect()
    }

    /// Return a reference to the current run configuration.
    pub fn config(&self) -> &RunConfig {
        &self.config
    }
}

/// Simple exact-match scorer.
pub fn exact_match(actual: &str, expected: &str) -> bool {
    actual.trim() == expected.trim()
}

/// Prefix-match scorer (actual starts with expected).
pub fn prefix_match(actual: &str, expected: &str) -> bool {
    actual.trim().starts_with(expected.trim())
}

/// Build a deterministic fixture inference function for testing.
pub fn fixture_infer(
    answers: HashMap<String, String>,
) -> impl Fn(&str, u64) -> (String, u64, u64) {
    move |input: &str, _seed: u64| {
        let out = answers.get(input).cloned().unwrap_or_default();
        (out, 10, 5)
    }
}
