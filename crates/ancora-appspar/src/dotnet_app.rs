/// .NET/C# sample app parity module.
///
/// Models the canonical .NET (Anthropic.Client) agent sample app.

#[derive(Debug, Clone, PartialEq)]
pub struct DotnetApp {
    pub name: String,
    pub framework: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotnetMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotnetTrace {
    pub trace_id: String,
    pub framework: String,
    pub messages: Vec<DotnetMessage>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DotnetAppError {
    EmptyContent,
    UnsupportedFramework(String),
}

impl std::fmt::Display for DotnetAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotnetAppError::EmptyContent => write!(f, "content must not be empty"),
            DotnetAppError::UnsupportedFramework(fw) => {
                write!(f, "unsupported framework: {}", fw)
            }
        }
    }
}

impl DotnetApp {
    pub fn new(
        name: impl Into<String>,
        framework: impl Into<String>,
    ) -> Result<Self, DotnetAppError> {
        let framework = framework.into();
        match framework.as_str() {
            "net8.0" | "net9.0" | "net6.0" => Ok(Self {
                name: name.into(),
                framework,
            }),
            other => Err(DotnetAppError::UnsupportedFramework(other.to_string())),
        }
    }

    pub fn run(&self, user_input: &str) -> Result<DotnetTrace, DotnetAppError> {
        if user_input.is_empty() {
            return Err(DotnetAppError::EmptyContent);
        }
        let user_msg = DotnetMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        let reply = format!(
            "[{}/{}] response: {}",
            self.name, self.framework, user_input
        );
        let assistant_msg = DotnetMessage {
            role: "assistant".to_string(),
            content: reply,
        };
        Ok(DotnetTrace {
            trace_id: format!("dotnet-trace-{}", user_input.len()),
            framework: self.framework.clone(),
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
