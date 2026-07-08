//! ancora-contrib: grader template
//!
//! Copy this module as the starting point for a new output-quality grader.
//! Rename `MyGrader` and implement `grade`.

/// The inputs fed to a grader for evaluation.
#[derive(Debug, Clone)]
pub struct GradeInput {
    /// The prompt or question that was posed.
    pub prompt: String,
    /// The model's generated response to evaluate.
    pub response: String,
    /// Optional ground-truth reference answer.
    pub reference: Option<String>,
    /// Optional metadata (e.g. task type, language).
    pub context: Vec<(String, String)>,
}

impl GradeInput {
    pub fn new(prompt: impl Into<String>, response: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            response: response.into(),
            reference: None,
            context: Vec::new(),
        }
    }

    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.push((key.into(), value.into()));
        self
    }
}

/// The grader's verdict for a single response.
#[derive(Debug, Clone, PartialEq)]
pub struct GradeResult {
    /// Score in [0.0, 1.0].
    pub score: f32,
    /// Human-readable explanation of the score.
    pub rationale: String,
    /// Whether this response passes the quality bar.
    pub passed: bool,
}

impl GradeResult {
    pub fn new(score: f32, rationale: impl Into<String>) -> Self {
        let passed = score >= 0.5;
        Self {
            score,
            rationale: rationale.into(),
            passed,
        }
    }
}

/// Errors a grader may return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraderError {
    InvalidInput(String),
    GradingFailed(String),
}

impl std::fmt::Display for GraderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraderError::InvalidInput(s) => write!(f, "invalid input: {s}"),
            GraderError::GradingFailed(s) => write!(f, "grading failed: {s}"),
        }
    }
}

impl std::error::Error for GraderError {}

/// Trait all grader plugins must implement.
pub trait GraderPlugin: Send + Sync {
    /// Stable identifier (e.g. "exact-match", "rouge-l", "llm-judge").
    fn grader_id(&self) -> &str;

    /// Human-readable description of the grading criterion.
    fn description(&self) -> &str;

    /// Grade a single response.
    fn grade(&self, input: &GradeInput) -> Result<GradeResult, GraderError>;
}

// ---------------------------------------------------------------------------
// Template implementation
// ---------------------------------------------------------------------------

/// Template grader: exact-match against the reference answer (case-insensitive).
pub struct MyGrader;

impl GraderPlugin for MyGrader {
    fn grader_id(&self) -> &str {
        // TODO: replace with a unique identifier.
        "my-exact-match"
    }

    fn description(&self) -> &str {
        // TODO: describe what this grader checks.
        "Scores 1.0 when the response exactly matches the reference (case-insensitive)."
    }

    fn grade(&self, input: &GradeInput) -> Result<GradeResult, GraderError> {
        let reference = input
            .reference
            .as_deref()
            .ok_or_else(|| GraderError::InvalidInput("reference answer is required".to_string()))?;

        // TODO: replace with your real scoring logic.
        let matched = input.response.trim().to_lowercase() == reference.trim().to_lowercase();
        let score = if matched { 1.0 } else { 0.0 };
        let rationale = if matched {
            "Response exactly matches the reference.".to_string()
        } else {
            "Response does not match the reference.".to_string()
        };
        Ok(GradeResult::new(score, rationale))
    }
}
