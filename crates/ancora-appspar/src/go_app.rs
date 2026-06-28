/// Go sample app parity module.
///
/// Models the canonical Go agent sample app: a simple request/response
/// loop with a named agent, a trace ID, and a list of messages.

#[derive(Debug, Clone, PartialEq)]
pub struct GoApp {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoTrace {
    pub trace_id: String,
    pub messages: Vec<GoMessage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoAppError {
    EmptyContent,
    UnknownRole(String),
}

impl std::fmt::Display for GoAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoAppError::EmptyContent => write!(f, "message content must not be empty"),
            GoAppError::UnknownRole(r) => write!(f, "unknown role: {}", r),
        }
    }
}

impl GoApp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "1.0.0".to_string(),
        }
    }

    pub fn create_message(
        &self,
        role: &str,
        content: &str,
    ) -> Result<GoMessage, GoAppError> {
        if content.is_empty() {
            return Err(GoAppError::EmptyContent);
        }
        match role {
            "user" | "assistant" | "system" => Ok(GoMessage {
                role: role.to_string(),
                content: content.to_string(),
            }),
            other => Err(GoAppError::UnknownRole(other.to_string())),
        }
    }

    pub fn run(&self, user_input: &str) -> Result<GoTrace, GoAppError> {
        let user_msg = self.create_message("user", user_input)?;
        let reply = format!("[{}] echo: {}", self.name, user_input);
        let assistant_msg = self.create_message("assistant", &reply)?;
        let trace_id = format!("go-trace-{}", user_input.len());
        Ok(GoTrace {
            trace_id,
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
