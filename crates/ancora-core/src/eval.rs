/// An evaluation case: an input, expected output, and a scorer name.
#[derive(Debug, Clone)]
pub struct EvalCase {
    pub id: String,
    pub input: String,
    pub expected: String,
    pub scorer: String,
}

impl EvalCase {
    pub fn new(
        id: impl Into<String>,
        input: impl Into<String>,
        expected: impl Into<String>,
        scorer: impl Into<String>,
    ) -> Self {
        Self { id: id.into(), input: input.into(), expected: expected.into(), scorer: scorer.into() }
    }
}

/// A scorer receives a candidate answer and the expected answer and returns
/// a score in `[0.0, 1.0]`.
pub trait EvalScorer: Send + Sync {
    fn name(&self) -> &str;
    fn score(&self, candidate: &str, expected: &str) -> f64;
}

/// Exact-match scorer: 1.0 if strings are equal, 0.0 otherwise.
pub struct ExactMatchScorer;

impl EvalScorer for ExactMatchScorer {
    fn name(&self) -> &str { "exact_match" }
    fn score(&self, candidate: &str, expected: &str) -> f64 {
        if candidate.trim() == expected.trim() { 1.0 } else { 0.0 }
    }
}

/// Contains-match scorer: 1.0 if expected is a substring of candidate.
pub struct ContainsScorer;

impl EvalScorer for ContainsScorer {
    fn name(&self) -> &str { "contains" }
    fn score(&self, candidate: &str, expected: &str) -> f64 {
        if candidate.contains(expected) { 1.0 } else { 0.0 }
    }
}
