//! Guardrail extension point - intercept and filter agent inputs and outputs.

/// The kind of content being checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentKind {
    Input,
    Output,
}

/// A piece of content submitted for guardrail inspection.
#[derive(Debug, Clone)]
pub struct GuardrailRequest {
    pub kind: ContentKind,
    pub text: String,
    /// Arbitrary context key-value pairs (e.g. "agent_id", "session_id").
    pub context: std::collections::HashMap<String, String>,
}

/// The decision returned by a guardrail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailDecision {
    /// Allow the content through unchanged.
    Allow,
    /// Allow but replace the content with an alternative.
    Rewrite(String),
    /// Block the content entirely.
    Block(String),
}

/// Error from a guardrail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailError {
    InternalError(String),
    Timeout,
}

impl std::fmt::Display for GuardrailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardrailError::InternalError(s) => write!(f, "guardrail internal error: {s}"),
            GuardrailError::Timeout => write!(f, "guardrail timed out"),
        }
    }
}

impl std::error::Error for GuardrailError {}

/// Trait that guardrail plugins must implement.
pub trait GuardrailPlugin: Send + Sync {
    fn guardrail_id(&self) -> &str;

    /// Inspect content and return an allow/rewrite/block decision.
    fn check(&self, req: GuardrailRequest) -> Result<GuardrailDecision, GuardrailError>;

    /// Optional priority (lower number = runs first in a chain).
    fn priority(&self) -> i32 {
        100
    }
}

/// A guardrail that blocks content containing a configurable banned word list.
pub struct BannedWordGuardrail {
    id: String,
    banned: Vec<String>,
}

impl BannedWordGuardrail {
    pub fn new(id: impl Into<String>, banned: Vec<String>) -> Self {
        Self {
            id: id.into(),
            banned,
        }
    }
}

impl GuardrailPlugin for BannedWordGuardrail {
    fn guardrail_id(&self) -> &str {
        &self.id
    }

    fn check(&self, req: GuardrailRequest) -> Result<GuardrailDecision, GuardrailError> {
        let lower = req.text.to_lowercase();
        for word in &self.banned {
            if lower.contains(word.as_str()) {
                return Ok(GuardrailDecision::Block(format!(
                    "content contains banned term: {word}"
                )));
            }
        }
        Ok(GuardrailDecision::Allow)
    }
}
