use std::collections::HashMap;
use std::sync::Arc;

use crate::eval::{EvalCase, EvalScorer};

/// Result of running a single rollout of one eval case.
#[derive(Debug, Clone)]
pub struct RolloutResult {
    pub case_id: String,
    pub rollout: usize,
    pub candidate: String,
    pub score: f64,
    pub passed: bool,
}

/// Aggregated results across all rollouts for one eval case.
#[derive(Debug, Clone)]
pub struct CaseResult {
    pub case_id: String,
    pub rollouts: Vec<RolloutResult>,
    pub pass_count: usize,
    pub n: usize,
}

impl CaseResult {
    pub fn pass_rate(&self) -> f64 {
        if self.n == 0 { 0.0 } else { self.pass_count as f64 / self.n as f64 }
    }
}

/// Function type used to generate a candidate answer from an input.
pub type GenerateFn = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// Runs N rollouts of each eval case using the provided scorers and generator.
pub struct EvalRunner {
    scorers: HashMap<String, Arc<dyn EvalScorer>>,
    n: usize,
}

impl EvalRunner {
    pub fn new(n: usize) -> Self {
        Self { scorers: HashMap::new(), n }
    }

    pub fn register_scorer(&mut self, scorer: Arc<dyn EvalScorer>) -> &mut Self {
        self.scorers.insert(scorer.name().to_owned(), scorer);
        self
    }

    pub fn run(&self, cases: &[EvalCase], generate: &GenerateFn) -> Vec<CaseResult> {
        cases.iter().map(|case| self.run_case(case, generate)).collect()
    }

    fn run_case(&self, case: &EvalCase, generate: &GenerateFn) -> CaseResult {
        let scorer = self.scorers.get(&case.scorer);
        let mut rollouts = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let candidate = generate(&case.input);
            let score = scorer.map(|s| s.score(&candidate, &case.expected)).unwrap_or(0.0);
            rollouts.push(RolloutResult {
                case_id: case.id.clone(),
                rollout: i,
                candidate,
                score,
                passed: score >= 0.5,
            });
        }
        let pass_count = rollouts.iter().filter(|r| r.passed).count();
        CaseResult { case_id: case.id.clone(), rollouts, pass_count, n: self.n }
    }
}
