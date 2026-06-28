/// Provider extension point - integrate an external LLM API.

/// A single message in a conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Role of a conversation participant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Inference parameters passed to the provider.
#[derive(Debug, Clone)]
pub struct GenerateRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stop_sequences: Vec<String>,
}

/// The provider's response.
#[derive(Debug, Clone)]
pub struct GenerateResponse {
    pub content: String,
    pub model: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    /// Whether the response was cut off by `max_tokens`.
    pub truncated: bool,
}

/// Error from a provider call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderError {
    Unavailable(String),
    RateLimited,
    InvalidRequest(String),
    AuthFailed,
    ModelNotFound(String),
    Unknown(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::Unavailable(s) => write!(f, "provider unavailable: {s}"),
            ProviderError::RateLimited => write!(f, "rate limited"),
            ProviderError::InvalidRequest(s) => write!(f, "invalid request: {s}"),
            ProviderError::AuthFailed => write!(f, "authentication failed"),
            ProviderError::ModelNotFound(m) => write!(f, "model not found: {m}"),
            ProviderError::Unknown(s) => write!(f, "unknown error: {s}"),
        }
    }
}

impl std::error::Error for ProviderError {}

/// Trait that provider plugins must implement.
pub trait ProviderPlugin: Send + Sync {
    /// Stable identifier for this provider (e.g. "openai", "anthropic").
    fn provider_id(&self) -> &str;

    /// Return a list of model identifiers this provider supports.
    fn list_models(&self) -> Vec<String>;

    /// Generate a completion synchronously.
    fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse, ProviderError>;
}

/// A no-op echo provider useful for tests.
pub struct EchoProvider {
    pub id: String,
}

impl EchoProvider {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl ProviderPlugin for EchoProvider {
    fn provider_id(&self) -> &str {
        &self.id
    }

    fn list_models(&self) -> Vec<String> {
        vec!["echo-1".to_string()]
    }

    fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        let last = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
        Ok(GenerateResponse {
            content: format!("echo: {last}"),
            model: req.model,
            input_tokens: last.len() as u32,
            output_tokens: last.len() as u32 + 6,
            truncated: false,
        })
    }
}
