/// A numeric score in [0.0, 1.0] returned by a grader.
#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    /// The numeric score between 0.0 (worst) and 1.0 (best).
    pub value: f64,
    /// Optional human-readable explanation of how the score was determined.
    pub rationale: Option<String>,
}

impl Score {
    pub fn new(value: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&value),
            "Score must be in [0.0, 1.0], got {}",
            value
        );
        Self {
            value,
            rationale: None,
        }
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }
}

/// The interface every grader must implement.
pub trait Grader {
    /// Grade a candidate answer against the expected reference.
    ///
    /// Returns a `Score` in [0.0, 1.0].
    fn grade(&self, candidate: &str, expected: &str) -> Score;

    /// A unique, stable name for this grader (used in result records).
    fn name(&self) -> &str;
}
