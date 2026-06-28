/// ancora-contrib: provider adapter template
///
/// Copy this module as the starting point for a new LLM provider plugin.
/// Replace all occurrences of `MyProvider` and `"my-provider"` with your own
/// identifier, then implement the two required methods.

/// A single conversation message.
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

/// Request payload for a generation call.
#[derive(Debug, Clone)]
pub struct GenerateRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

/// Successful generation response.
#[derive(Debug, Clone)]
pub struct GenerateResponse {
    pub content: String,
    pub model: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub truncated: bool,
}

/// Errors a provider may return.
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

/// Trait all provider adapters must implement.
pub trait ProviderAdapter: Send + Sync {
    /// Stable, lowercase, hyphenated identifier (e.g. "acme-cloud").
    fn provider_id(&self) -> &str;

    /// Return supported model identifiers.
    fn list_models(&self) -> Vec<String>;

    /// Synchronous generation call.
    fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse, ProviderError>;
}

// ---------------------------------------------------------------------------
// Template implementation - rename `MyProvider` and fill in the real logic.
// ---------------------------------------------------------------------------

/// Template provider: echoes the last user message back.
/// Use this as a test stand-in before wiring a real API.
pub struct MyProvider {
    /// Replace with whatever configuration your provider requires.
    pub api_key: String,
}

impl MyProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }
}

impl ProviderAdapter for MyProvider {
    fn provider_id(&self) -> &str {
        // TODO: replace with your provider's identifier.
        "my-provider"
    }

    fn list_models(&self) -> Vec<String> {
        // TODO: return the real model list.
        vec!["my-provider-v1".to_string()]
    }

    fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse, ProviderError> {
        if req.model != "my-provider-v1" {
            return Err(ProviderError::ModelNotFound(req.model));
        }
        // TODO: replace with a real HTTP call to your provider.
        let last = req.messages.last().map(|m| m.content.as_str()).unwrap_or("");
        Ok(GenerateResponse {
            content: format!("my-provider echo: {last}"),
            model: "my-provider-v1".to_string(),
            input_tokens: last.len() as u32,
            output_tokens: last.len() as u32 + 16,
            truncated: false,
        })
    }
}
