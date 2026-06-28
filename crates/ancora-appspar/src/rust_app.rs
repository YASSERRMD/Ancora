/// Rust sample app parity module.
///
/// Models the canonical Rust (anthropic-rs) agent sample app.

#[derive(Debug, Clone, PartialEq)]
pub struct RustApp {
    pub name: String,
    pub edition: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustTrace {
    pub trace_id: String,
    pub edition: u16,
    pub messages: Vec<RustMessage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustAppError {
    EmptyContent,
    UnsupportedEdition(u16),
}

impl std::fmt::Display for RustAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustAppError::EmptyContent => write!(f, "content must not be empty"),
            RustAppError::UnsupportedEdition(e) => write!(f, "unsupported edition: {}", e),
        }
    }
}

impl RustApp {
    pub fn new(name: impl Into<String>, edition: u16) -> Result<Self, RustAppError> {
        match edition {
            2018 | 2021 | 2024 => Ok(Self {
                name: name.into(),
                edition,
            }),
            other => Err(RustAppError::UnsupportedEdition(other)),
        }
    }

    pub fn run(&self, user_input: &str) -> Result<RustTrace, RustAppError> {
        if user_input.is_empty() {
            return Err(RustAppError::EmptyContent);
        }
        let user_msg = RustMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        let reply = format!("[{}:edition{}] output: {}", self.name, self.edition, user_input);
        let assistant_msg = RustMessage {
            role: "assistant".to_string(),
            content: reply,
        };
        Ok(RustTrace {
            trace_id: format!("rust-trace-{}", user_input.len()),
            edition: self.edition,
            messages: vec![user_msg, assistant_msg],
        })
    }
}

pub fn feature_list() -> Vec<&'static str> {
    vec![
        "streaming",
        "tool_calls",
        "structured_output",
        "guardrails",
        "tracing",
    ]
}
