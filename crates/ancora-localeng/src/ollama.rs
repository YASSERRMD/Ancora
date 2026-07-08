/// Integration with Ollama (https://ollama.ai).
///
/// Ollama exposes a REST API on port 11434 by default. This module
/// provides a typed client with a pluggable transport so tests run
/// fully offline using mocks.
use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default Ollama API base URL.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:11434";

/// Errors from the Ollama client.
#[derive(Debug)]
pub enum OllamaError {
    Unreachable(String),
    ModelNotFound(String),
    BadResponse(String),
}

impl std::fmt::Display for OllamaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OllamaError::Unreachable(u) => write!(f, "ollama unreachable: {}", u),
            OllamaError::ModelNotFound(m) => write!(f, "model not found: {}", m),
            OllamaError::BadResponse(r) => write!(f, "bad response: {}", r),
        }
    }
}

/// A list of locally available models returned by Ollama.
#[derive(Debug, Clone)]
pub struct OllamaModelList {
    pub models: Vec<String>,
}

impl OllamaModelList {
    pub fn contains(&self, name: &str) -> bool {
        self.models.iter().any(|m| m == name)
    }
}

/// Transport abstraction for Ollama.
pub trait OllamaTransport {
    fn list_models(&self, endpoint: &str) -> Result<OllamaModelList, OllamaError>;
    fn generate(
        &self,
        endpoint: &str,
        model: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, OllamaError>;
    fn ping(&self, endpoint: &str) -> Result<bool, OllamaError>;
}

/// Ollama client backed by a pluggable transport.
pub struct OllamaClient<T: OllamaTransport> {
    pub config: EngineConfig,
    pub model: String,
    transport: T,
}

impl<T: OllamaTransport> OllamaClient<T> {
    pub fn new(config: EngineConfig, model: &str, transport: T) -> Self {
        OllamaClient {
            config,
            model: model.to_string(),
            transport,
        }
    }

    pub fn endpoint(&self) -> &str {
        self.config.endpoint.as_deref().unwrap_or(DEFAULT_ENDPOINT)
    }

    pub fn list_models(&self) -> Result<OllamaModelList, OllamaError> {
        self.transport.list_models(self.endpoint())
    }

    pub fn complete(&self, request: &CompletionRequest) -> Result<CompletionResult, OllamaError> {
        self.transport
            .generate(self.endpoint(), &self.model, request)
    }

    pub fn ping(&self) -> Result<bool, OllamaError> {
        self.transport.ping(self.endpoint())
    }
}

/// Default config for Ollama.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::Ollama).with_endpoint(DEFAULT_ENDPOINT)
}
