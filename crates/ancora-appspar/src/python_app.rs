/// Python sample app parity module.
///
/// Models the canonical Python (anthropic-sdk) agent sample app.

#[derive(Debug, Clone, PartialEq)]
pub struct PythonApp {
    pub name: String,
    pub model: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PythonMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PythonTrace {
    pub trace_id: String,
    pub model: String,
    pub messages: Vec<PythonMessage>,
    pub input_tokens: usize,
    pub output_tokens: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PythonAppError {
    EmptyContent,
    InvalidModel(String),
}

impl std::fmt::Display for PythonAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PythonAppError::EmptyContent => write!(f, "content must not be empty"),
            PythonAppError::InvalidModel(m) => write!(f, "invalid model: {}", m),
        }
    }
}

impl PythonApp {
    pub fn new(name: impl Into<String>, model: impl Into<String>) -> Result<Self, PythonAppError> {
        let model = model.into();
        if model.is_empty() {
            return Err(PythonAppError::InvalidModel(model));
        }
        Ok(Self {
            name: name.into(),
            model,
        })
    }

    pub fn run(&self, user_input: &str) -> Result<PythonTrace, PythonAppError> {
        if user_input.is_empty() {
            return Err(PythonAppError::EmptyContent);
        }
        let user_msg = PythonMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        let reply = format!("[{}] processed: {}", self.name, user_input);
        let assistant_msg = PythonMessage {
            role: "assistant".to_string(),
            content: reply,
        };
        let input_tokens = user_input.split_whitespace().count();
        let output_tokens = input_tokens + 3;
        Ok(PythonTrace {
            trace_id: format!("py-trace-{}", input_tokens),
            model: self.model.clone(),
            messages: vec![user_msg, assistant_msg],
            input_tokens,
            output_tokens,
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
