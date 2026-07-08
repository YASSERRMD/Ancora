//! ancora-contrib: guardrail template
//!
//! Copy this module as the starting point for a new safety guardrail.
//! Rename `MyGuardrail` and implement `check`.

/// The text to be evaluated by the guardrail.
#[derive(Debug, Clone)]
pub struct GuardrailInput {
    /// Raw text of the model output (or user input) to check.
    pub text: String,
    /// Optional conversational context (prior turns, etc.).
    pub context: Vec<String>,
}

impl GuardrailInput {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            context: Vec::new(),
        }
    }

    pub fn with_context_turn(mut self, turn: impl Into<String>) -> Self {
        self.context.push(turn.into());
        self
    }
}

/// Severity level when a guardrail fires.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        };
        write!(f, "{s}")
    }
}

/// The verdict returned by a guardrail.
#[derive(Debug, Clone, PartialEq)]
pub enum GuardrailVerdict {
    /// Text is safe; pass it through.
    Allow,
    /// Text should be blocked. Contains a reason and severity.
    Block { reason: String, severity: Severity },
    /// Text should be modified. Contains a sanitised replacement.
    Redact { replacement: String, reason: String },
}

impl GuardrailVerdict {
    pub fn is_allowed(&self) -> bool {
        matches!(self, GuardrailVerdict::Allow)
    }
}

/// Errors a guardrail may return.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailError {
    CheckFailed(String),
}

impl std::fmt::Display for GuardrailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardrailError::CheckFailed(s) => write!(f, "guardrail check failed: {s}"),
        }
    }
}

impl std::error::Error for GuardrailError {}

/// Trait all guardrail plugins must implement.
pub trait GuardrailPlugin: Send + Sync {
    /// Stable identifier (e.g. "pii-detector", "toxicity-filter").
    fn guardrail_id(&self) -> &str;

    /// Human-readable description of what this guardrail protects against.
    fn description(&self) -> &str;

    /// Evaluate the input and return a verdict.
    fn check(&self, input: &GuardrailInput) -> Result<GuardrailVerdict, GuardrailError>;
}

// ---------------------------------------------------------------------------
// Template implementation
// ---------------------------------------------------------------------------

/// Template guardrail: blocks any text that contains a configurable keyword list.
pub struct MyGuardrail {
    pub blocked_keywords: Vec<String>,
}

impl MyGuardrail {
    pub fn new(blocked_keywords: Vec<impl Into<String>>) -> Self {
        Self {
            blocked_keywords: blocked_keywords.into_iter().map(|k| k.into()).collect(),
        }
    }
}

impl GuardrailPlugin for MyGuardrail {
    fn guardrail_id(&self) -> &str {
        // TODO: replace with your guardrail's identifier.
        "my-keyword-block"
    }

    fn description(&self) -> &str {
        // TODO: describe what the guardrail detects.
        "Blocks output that contains any of the configured keywords."
    }

    fn check(&self, input: &GuardrailInput) -> Result<GuardrailVerdict, GuardrailError> {
        let lower = input.text.to_lowercase();
        // TODO: replace with your real detection logic.
        for kw in &self.blocked_keywords {
            if lower.contains(kw.as_str()) {
                return Ok(GuardrailVerdict::Block {
                    reason: format!("blocked keyword detected: {kw}"),
                    severity: Severity::High,
                });
            }
        }
        Ok(GuardrailVerdict::Allow)
    }
}
