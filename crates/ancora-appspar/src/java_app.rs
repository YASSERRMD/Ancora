//! Java sample app parity module.
//!
//! Models the canonical Java (anthropic-java) agent sample app.

#[derive(Debug, Clone, PartialEq)]
pub struct JavaApp {
    pub name: String,
    pub java_version: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JavaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JavaTrace {
    pub trace_id: String,
    pub java_version: u8,
    pub messages: Vec<JavaMessage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JavaAppError {
    EmptyContent,
    UnsupportedJavaVersion(u8),
}

impl std::fmt::Display for JavaAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JavaAppError::EmptyContent => write!(f, "content must not be empty"),
            JavaAppError::UnsupportedJavaVersion(v) => {
                write!(f, "unsupported java version: {}", v)
            }
        }
    }
}

impl JavaApp {
    pub fn new(name: impl Into<String>, java_version: u8) -> Result<Self, JavaAppError> {
        if java_version < 11 {
            return Err(JavaAppError::UnsupportedJavaVersion(java_version));
        }
        Ok(Self {
            name: name.into(),
            java_version,
        })
    }

    pub fn run(&self, user_input: &str) -> Result<JavaTrace, JavaAppError> {
        if user_input.is_empty() {
            return Err(JavaAppError::EmptyContent);
        }
        let user_msg = JavaMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        let reply = format!(
            "[{}:java{}] answer: {}",
            self.name, self.java_version, user_input
        );
        let assistant_msg = JavaMessage {
            role: "assistant".to_string(),
            content: reply,
        };
        Ok(JavaTrace {
            trace_id: format!("java-trace-{}", user_input.len()),
            java_version: self.java_version,
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
