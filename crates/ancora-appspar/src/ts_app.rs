/// TypeScript/Node sample app parity module.
///
/// Models the canonical TypeScript (anthropic-sdk) agent sample app.

#[derive(Debug, Clone, PartialEq)]
pub struct TsApp {
    pub name: String,
    pub sdk_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsTrace {
    pub trace_id: String,
    pub messages: Vec<TsMessage>,
    pub stop_reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsAppError {
    EmptyContent,
    InvalidSdkVersion,
}

impl std::fmt::Display for TsAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAppError::EmptyContent => write!(f, "content must not be empty"),
            TsAppError::InvalidSdkVersion => write!(f, "sdk version must not be empty"),
        }
    }
}

impl TsApp {
    pub fn new(name: impl Into<String>, sdk_version: impl Into<String>) -> Result<Self, TsAppError> {
        let sdk_version = sdk_version.into();
        if sdk_version.is_empty() {
            return Err(TsAppError::InvalidSdkVersion);
        }
        Ok(Self {
            name: name.into(),
            sdk_version,
        })
    }

    pub fn run(&self, user_input: &str) -> Result<TsTrace, TsAppError> {
        if user_input.is_empty() {
            return Err(TsAppError::EmptyContent);
        }
        let user_msg = TsMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        let reply = format!("[{}@{}] reply: {}", self.name, self.sdk_version, user_input);
        let assistant_msg = TsMessage {
            role: "assistant".to_string(),
            content: reply,
        };
        Ok(TsTrace {
            trace_id: format!("ts-trace-{}", user_input.len()),
            messages: vec![user_msg, assistant_msg],
            stop_reason: "end_turn".to_string(),
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
