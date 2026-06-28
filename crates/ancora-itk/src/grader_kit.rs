/// Conformance kit for grader extensions.

/// A grading request: a question, reference answer, and candidate answer.
#[derive(Debug, Clone)]
pub struct GradeRequest {
    pub question: String,
    pub reference: String,
    pub candidate: String,
}

/// A grading result: score in [0.0, 1.0] and optional rationale.
#[derive(Debug, Clone)]
pub struct GradeResult {
    pub score: f64,
    pub rationale: Option<String>,
}

/// Trait that every grader extension must satisfy.
pub trait Grader {
    fn name(&self) -> &str;
    fn grade(&self, request: &GradeRequest) -> Result<GradeResult, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`Grader`].
pub struct GraderKit;

impl GraderKit {
    pub fn new() -> Self {
        GraderKit
    }

    pub fn run<G: Grader>(&self, grader: &G) -> Vec<CheckResult> {
        vec![
            self.check_name(grader),
            self.check_perfect_match(grader),
            self.check_score_bounds(grader),
        ]
    }

    fn check_name<G: Grader>(&self, grader: &G) -> CheckResult {
        if grader.name().is_empty() {
            CheckResult {
                name: "grader_name_nonempty".into(),
                passed: false,
                message: "Grader name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "grader_name_nonempty".into(),
                passed: true,
                message: format!("Grader name: {}", grader.name()),
            }
        }
    }

    fn check_perfect_match<G: Grader>(&self, grader: &G) -> CheckResult {
        let req = GradeRequest {
            question: "What is 2 + 2?".into(),
            reference: "4".into(),
            candidate: "4".into(),
        };
        match grader.grade(&req) {
            Ok(result) if result.score >= 0.9 => CheckResult {
                name: "grader_perfect_match_scores_high".into(),
                passed: true,
                message: format!("Score for identical answer: {:.2}", result.score),
            },
            Ok(result) => CheckResult {
                name: "grader_perfect_match_scores_high".into(),
                passed: false,
                message: format!(
                    "Expected score >= 0.9 for identical answer, got {:.2}",
                    result.score
                ),
            },
            Err(e) => CheckResult {
                name: "grader_perfect_match_scores_high".into(),
                passed: false,
                message: format!("grade() errored: {e}"),
            },
        }
    }

    fn check_score_bounds<G: Grader>(&self, grader: &G) -> CheckResult {
        let req = GradeRequest {
            question: "What is 2 + 2?".into(),
            reference: "4".into(),
            candidate: "banana".into(),
        };
        match grader.grade(&req) {
            Ok(result) if (0.0..=1.0).contains(&result.score) => CheckResult {
                name: "grader_score_in_bounds".into(),
                passed: true,
                message: format!("Score {:.2} is in [0, 1]", result.score),
            },
            Ok(result) => CheckResult {
                name: "grader_score_in_bounds".into(),
                passed: false,
                message: format!("Score {:.2} out of [0, 1] range", result.score),
            },
            Err(e) => CheckResult {
                name: "grader_score_in_bounds".into(),
                passed: false,
                message: format!("grade() errored: {e}"),
            },
        }
    }
}

impl Default for GraderKit {
    fn default() -> Self {
        Self::new()
    }
}
