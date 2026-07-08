/// Grader extension point - score or rank agent responses.

/// A candidate response to be graded.
#[derive(Debug, Clone)]
pub struct GradeRequest {
    /// The original prompt or question.
    pub prompt: String,
    /// The candidate response.
    pub response: String,
    /// Optional reference / ground truth.
    pub reference: Option<String>,
    /// Arbitrary metadata.
    pub metadata: std::collections::HashMap<String, String>,
}

/// The numeric score assigned to a response.
#[derive(Debug, Clone, PartialEq)]
pub struct Grade {
    /// Score in [0.0, 1.0].
    pub score: f32,
    /// Optional explanation.
    pub rationale: Option<String>,
    /// Whether the grader considers the response passing (score >= threshold).
    pub pass: bool,
}

impl Grade {
    pub fn new(score: f32, rationale: Option<String>, pass_threshold: f32) -> Self {
        let score = score.clamp(0.0, 1.0);
        Self {
            pass: score >= pass_threshold,
            score,
            rationale,
        }
    }
}

/// Error from a grader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraderError {
    InvalidInput(String),
    InternalError(String),
}

impl std::fmt::Display for GraderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraderError::InvalidInput(s) => write!(f, "invalid grader input: {s}"),
            GraderError::InternalError(s) => write!(f, "grader internal error: {s}"),
        }
    }
}

impl std::error::Error for GraderError {}

/// Trait that grader plugins must implement.
pub trait GraderPlugin: Send + Sync {
    fn grader_id(&self) -> &str;

    /// Grade a single response.
    fn grade(&self, req: GradeRequest) -> Result<Grade, GraderError>;

    /// The default pass threshold for this grader (in [0.0, 1.0]).
    fn pass_threshold(&self) -> f32 {
        0.7
    }
}

/// A simple length-ratio grader: score = response_len / reference_len (capped at 1.0).
pub struct LengthRatioGrader {
    id: String,
    threshold: f32,
}

impl LengthRatioGrader {
    pub fn new(id: impl Into<String>, threshold: f32) -> Self {
        Self {
            id: id.into(),
            threshold: threshold.clamp(0.0, 1.0),
        }
    }
}

impl GraderPlugin for LengthRatioGrader {
    fn grader_id(&self) -> &str {
        &self.id
    }

    fn pass_threshold(&self) -> f32 {
        self.threshold
    }

    fn grade(&self, req: GradeRequest) -> Result<Grade, GraderError> {
        let reference = req.reference.as_deref().ok_or_else(|| {
            GraderError::InvalidInput("reference is required for LengthRatioGrader".into())
        })?;
        if reference.is_empty() {
            return Err(GraderError::InvalidInput("reference is empty".into()));
        }
        let ratio = (req.response.len() as f32 / reference.len() as f32).min(1.0);
        Ok(Grade::new(
            ratio,
            Some(format!(
                "response length {} vs reference length {}",
                req.response.len(),
                reference.len()
            )),
            self.threshold,
        ))
    }
}
