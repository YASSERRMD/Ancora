/// Evaluation module for scoring agent outputs against a test suite.

/// A single evaluation case with an expected outcome.
#[derive(Debug, Clone)]
pub struct EvalCase {
    pub id: String,
    pub input: String,
    pub expected: String,
}

impl EvalCase {
    pub fn new(
        id: impl Into<String>,
        input: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            input: input.into(),
            expected: expected.into(),
        }
    }
}

/// Score for a single eval case.
#[derive(Debug, Clone, PartialEq)]
pub struct EvalScore {
    pub case_id: String,
    pub passed: bool,
    pub score: f64,
}

impl EvalScore {
    pub fn pass(case_id: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            passed: true,
            score: 1.0,
        }
    }

    pub fn fail(case_id: impl Into<String>, score: f64) -> Self {
        Self {
            case_id: case_id.into(),
            passed: false,
            score,
        }
    }
}

/// Result of running a full eval suite.
#[derive(Debug)]
pub struct EvalResult {
    pub suite_id: String,
    pub scores: Vec<EvalScore>,
}

impl EvalResult {
    pub fn new(suite_id: impl Into<String>) -> Self {
        Self {
            suite_id: suite_id.into(),
            scores: Vec::new(),
        }
    }

    pub fn add_score(&mut self, score: EvalScore) {
        self.scores.push(score);
    }

    pub fn pass_rate(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        let passed = self.scores.iter().filter(|s| s.passed).count();
        passed as f64 / self.scores.len() as f64
    }

    pub fn mean_score(&self) -> f64 {
        if self.scores.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.scores.iter().map(|s| s.score).sum();
        sum / self.scores.len() as f64
    }

    pub fn all_passed(&self) -> bool {
        self.scores.iter().all(|s| s.passed)
    }
}

/// A local judge that scores outputs by exact match.
pub struct ExactMatchJudge;

impl ExactMatchJudge {
    pub fn score(&self, case: &EvalCase, output: &str) -> EvalScore {
        let passed = output.trim() == case.expected.trim();
        if passed {
            EvalScore::pass(&case.id)
        } else {
            EvalScore::fail(&case.id, 0.0)
        }
    }
}

/// Build a simple test suite and run it with the local judge.
pub fn run_eval_suite(suite_id: &str, cases: &[(EvalCase, String)]) -> EvalResult {
    let judge = ExactMatchJudge;
    let mut result = EvalResult::new(suite_id);
    for (case, output) in cases {
        result.add_score(judge.score(case, output));
    }
    result
}

/// Build a default offline eval suite for testing.
pub fn default_eval_suite() -> Vec<(EvalCase, String)> {
    vec![
        (EvalCase::new("c1", "What is 2+2?", "4"), "4".to_string()),
        (
            EvalCase::new("c2", "Capital of France?", "Paris"),
            "Paris".to_string(),
        ),
        (
            EvalCase::new("c3", "Opposite of hot?", "cold"),
            "cold".to_string(),
        ),
    ]
}
