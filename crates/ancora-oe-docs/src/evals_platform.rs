//! Evaluation platform primitives for running agent evals.

/// Defines the input/output contract for an evaluation.
#[derive(Debug, Clone)]
pub struct EvalSpec {
    pub id: String,
    pub name: String,
    pub description: String,
    pub grader_id: String,
}

impl EvalSpec {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        grader_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            grader_id: grader_id.into(),
        }
    }
}

/// The result of running an evaluation against a single sample.
#[derive(Debug, Clone)]
pub struct EvalResult {
    pub eval_id: String,
    pub sample_id: String,
    pub score: f64,
    pub passed: bool,
    pub metadata: Vec<(String, String)>,
}

impl EvalResult {
    pub fn new(
        eval_id: impl Into<String>,
        sample_id: impl Into<String>,
        score: f64,
        threshold: f64,
    ) -> Self {
        let passed = score >= threshold;
        Self {
            eval_id: eval_id.into(),
            sample_id: sample_id.into(),
            score,
            passed,
            metadata: Vec::new(),
        }
    }
}

/// Aggregates results from a batch eval run.
#[derive(Debug, Default)]
pub struct EvalRunSummary {
    pub results: Vec<EvalResult>,
}

impl EvalRunSummary {
    pub fn add(&mut self, result: EvalResult) {
        self.results.push(result);
    }

    pub fn pass_rate(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        let passed = self.results.iter().filter(|r| r.passed).count();
        passed as f64 / self.results.len() as f64
    }

    pub fn mean_score(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        self.results.iter().map(|r| r.score).sum::<f64>() / self.results.len() as f64
    }
}
