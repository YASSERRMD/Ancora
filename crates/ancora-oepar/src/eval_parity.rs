//! Eval run parity - ensures eval execution produces consistent results across SDKs.

use std::collections::HashMap;

/// An individual evaluation case.
#[derive(Debug, Clone)]
pub struct EvalCase {
    pub id: String,
    pub input: String,
    pub expected_output: String,
    pub metadata: HashMap<String, String>,
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
            expected_output: expected.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// The result of running a single eval case.
#[derive(Debug, Clone)]
pub struct EvalResult {
    pub case_id: String,
    pub language: String,
    pub score: f64,
    pub passed: bool,
    pub actual_output: String,
}

impl EvalResult {
    pub fn new(
        case_id: impl Into<String>,
        language: impl Into<String>,
        score: f64,
        actual_output: impl Into<String>,
        pass_threshold: f64,
    ) -> Self {
        let passed = score >= pass_threshold;
        Self {
            case_id: case_id.into(),
            language: language.into(),
            score,
            passed,
            actual_output: actual_output.into(),
        }
    }
}

/// Summary of an eval run across all cases.
#[derive(Debug, Clone)]
pub struct EvalRunSummary {
    pub language: String,
    pub total_cases: usize,
    pub passed: usize,
    pub failed: usize,
    pub mean_score: f64,
}

impl EvalRunSummary {
    pub fn from_results(language: impl Into<String>, results: &[EvalResult]) -> Self {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let mean_score = if total == 0 {
            0.0
        } else {
            results.iter().map(|r| r.score).sum::<f64>() / total as f64
        };
        Self {
            language: language.into(),
            total_cases: total,
            passed,
            failed: total - passed,
            mean_score,
        }
    }

    pub fn pass_rate(&self) -> f64 {
        if self.total_cases == 0 {
            0.0
        } else {
            self.passed as f64 / self.total_cases as f64
        }
    }
}

/// Shared eval dataset used across all languages.
pub fn shared_eval_dataset() -> Vec<EvalCase> {
    vec![
        EvalCase::new("case-001", "What is 2+2?", "4").with_metadata("category", "math"),
        EvalCase::new(
            "case-002",
            "Summarize: The sky is blue.",
            "The sky is blue.",
        )
        .with_metadata("category", "summarization"),
        EvalCase::new("case-003", "Translate 'hello' to Spanish.", "hola")
            .with_metadata("category", "translation"),
    ]
}

/// Simulate running eval cases for a given language (deterministic stub).
pub fn run_eval(language: impl Into<String>, cases: &[EvalCase]) -> Vec<EvalResult> {
    let lang = language.into();
    cases
        .iter()
        .map(|c| {
            // Deterministic stub: always returns perfect score.
            EvalResult::new(&c.id, &lang, 1.0, &c.expected_output, 0.8)
        })
        .collect()
}

/// Check that eval summaries across languages are within tolerance.
pub fn check_eval_parity(summaries: &[EvalRunSummary], score_tolerance: f64) -> Vec<String> {
    let mut issues = Vec::new();
    if let Some(first) = summaries.first() {
        for other in summaries.iter().skip(1) {
            if first.total_cases != other.total_cases {
                issues.push(format!(
                    "total_cases mismatch: {:?}={} vs {:?}={}",
                    first.language, first.total_cases, other.language, other.total_cases
                ));
            }
            let score_diff = (first.mean_score - other.mean_score).abs();
            if score_diff > score_tolerance {
                issues.push(format!(
                    "mean_score differs by {:.4} (tolerance {:.4}): {:?} vs {:?}",
                    score_diff, score_tolerance, first.language, other.language
                ));
            }
        }
    }
    issues
}
