/// Integration with llama.cpp HTTP server (server mode).
///
/// The llama.cpp server exposes an OpenAI-compatible HTTP API at a
/// configurable host:port. This module provides a typed client and
/// mock-friendly abstractions for use in offline tests.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default endpoint for a locally-started llama.cpp server.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8080";

/// Errors surfaced by the llama.cpp server client.
#[derive(Debug)]
pub enum LlamaServerError {
    Unreachable(String),
    BadResponse(String),
    Timeout,
}

impl std::fmt::Display for LlamaServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlamaServerError::Unreachable(u) => write!(f, "llama.cpp server unreachable: {}", u),
            LlamaServerError::BadResponse(r) => write!(f, "bad response: {}", r),
            LlamaServerError::Timeout => write!(f, "request timed out"),
        }
    }
}

/// A mock-friendly transport layer.
pub trait LlamaServerTransport {
    fn post_completion(
        &self,
        endpoint: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, LlamaServerError>;

    fn get_health(&self, endpoint: &str) -> Result<bool, LlamaServerError>;
}

/// Real HTTP transport (stub — actual HTTP is feature-gated behind a
/// cargo feature that requires an http client crate).
pub struct HttpTransport;

impl LlamaServerTransport for HttpTransport {
    fn post_completion(
        &self,
        _endpoint: &str,
        _request: &CompletionRequest,
    ) -> Result<CompletionResult, LlamaServerError> {
        Err(LlamaServerError::Unreachable(
            "real HTTP transport not enabled in this build".to_string(),
        ))
    }

    fn get_health(&self, _endpoint: &str) -> Result<bool, LlamaServerError> {
        Err(LlamaServerError::Unreachable(
            "real HTTP transport not enabled in this build".to_string(),
        ))
    }
}

/// Client for the llama.cpp server.
pub struct LlamaServerClient<T: LlamaServerTransport> {
    pub config: EngineConfig,
    transport: T,
}

impl<T: LlamaServerTransport> LlamaServerClient<T> {
    pub fn new(config: EngineConfig, transport: T) -> Self {
        LlamaServerClient { config, transport }
    }

    pub fn endpoint(&self) -> &str {
        self.config
            .endpoint
            .as_deref()
            .unwrap_or(DEFAULT_ENDPOINT)
    }

    pub fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, LlamaServerError> {
        self.transport.post_completion(self.endpoint(), request)
    }

    pub fn health(&self) -> Result<bool, LlamaServerError> {
        self.transport.get_health(self.endpoint())
    }
}

/// Construct a default config targeting the llama.cpp server engine.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::LlamaCppServer)
        .with_endpoint(DEFAULT_ENDPOINT)
}
