/// Integration with LM Studio (https://lmstudio.ai).
///
/// LM Studio runs a local OpenAI-compatible server on port 1234.
/// This module provides a typed client with pluggable transport.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default LM Studio endpoint.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:1234";

/// Errors from the LM Studio client.
#[derive(Debug)]
pub enum LmStudioError {
    Unreachable(String),
    NoModelLoaded,
    BadResponse(String),
}

impl std::fmt::Display for LmStudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LmStudioError::Unreachable(u) => write!(f, "lm studio unreachable: {}", u),
            LmStudioError::NoModelLoaded => write!(f, "no model loaded in lm studio"),
            LmStudioError::BadResponse(r) => write!(f, "bad response: {}", r),
        }
    }
}

/// Model info returned by LM Studio.
#[derive(Debug, Clone)]
pub struct LmStudioModel {
    pub id: String,
    pub object: String,
}

/// Transport abstraction for LM Studio.
pub trait LmStudioTransport {
    fn list_models(&self, endpoint: &str) -> Result<Vec<LmStudioModel>, LmStudioError>;
    fn complete(
        &self,
        endpoint: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, LmStudioError>;
    fn health(&self, endpoint: &str) -> Result<bool, LmStudioError>;
}

/// LM Studio client.
pub struct LmStudioClient<T: LmStudioTransport> {
    pub config: EngineConfig,
    transport: T,
}

impl<T: LmStudioTransport> LmStudioClient<T> {
    pub fn new(config: EngineConfig, transport: T) -> Self {
        LmStudioClient { config, transport }
    }

    pub fn endpoint(&self) -> &str {
        self.config.endpoint.as_deref().unwrap_or(DEFAULT_ENDPOINT)
    }

    pub fn list_models(&self) -> Result<Vec<LmStudioModel>, LmStudioError> {
        self.transport.list_models(self.endpoint())
    }

    pub fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, LmStudioError> {
        self.transport.complete(self.endpoint(), request)
    }

    pub fn health(&self) -> Result<bool, LmStudioError> {
        self.transport.health(self.endpoint())
    }
}

/// Default config for LM Studio.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::LmStudio).with_endpoint(DEFAULT_ENDPOINT)
}
