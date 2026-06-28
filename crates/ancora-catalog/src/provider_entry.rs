use crate::metadata::Metadata;

/// Supported model provider backends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderBackend {
    OpenAiCompatible,
    Anthropic,
    Gemini,
    Custom(String),
}

/// A catalog entry describing an LLM or embedding provider.
#[derive(Debug, Clone)]
pub struct ProviderEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub backend: ProviderBackend,
    /// Models offered by this provider, e.g. "claude-3-5-sonnet-latest".
    pub models: Vec<String>,
    /// Name of the environment variable that holds the API key.
    pub api_key_env: Option<String>,
    pub metadata: Metadata,
}

impl ProviderEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        backend: ProviderBackend,
        metadata: Metadata,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            backend,
            models: Vec::new(),
            api_key_env: None,
            metadata,
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.models.push(model.into());
        self
    }

    pub fn with_api_key_env(mut self, env: impl Into<String>) -> Self {
        self.api_key_env = Some(env.into());
        self
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty()
    }
}
