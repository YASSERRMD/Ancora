/// Integration with SGLang (https://github.com/sgl-project/sglang).
///
/// SGLang provides a high-throughput serving runtime with its own HTTP API.
/// This module wraps it with a typed client and mock transport.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default SGLang endpoint.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:30000";

/// Errors from the SGLang client.
#[derive(Debug)]
pub enum SglangError {
    Unreachable(String),
    BadRequest(String),
    BadResponse(String),
}

impl std::fmt::Display for SglangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SglangError::Unreachable(u) => write!(f, "sglang unreachable: {}", u),
            SglangError::BadRequest(r) => write!(f, "bad request: {}", r),
            SglangError::BadResponse(r) => write!(f, "bad response: {}", r),
        }
    }
}

/// Runtime statistics returned by the SGLang server.
#[derive(Debug, Clone)]
pub struct SglangStats {
    pub num_requests_running: usize,
    pub num_requests_waiting: usize,
    pub gpu_utilization: f32,
}

/// Transport abstraction for SGLang.
pub trait SglangTransport {
    fn generate(
        &self,
        endpoint: &str,
        model: &str,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, SglangError>;

    fn stats(&self, endpoint: &str) -> Result<SglangStats, SglangError>;
    fn health(&self, endpoint: &str) -> Result<bool, SglangError>;
}

/// SGLang client.
pub struct SglangClient<T: SglangTransport> {
    pub config: EngineConfig,
    pub model: String,
    transport: T,
}

impl<T: SglangTransport> SglangClient<T> {
    pub fn new(config: EngineConfig, model: &str, transport: T) -> Self {
        SglangClient {
            config,
            model: model.to_string(),
            transport,
        }
    }

    pub fn endpoint(&self) -> &str {
        self.config.endpoint.as_deref().unwrap_or(DEFAULT_ENDPOINT)
    }

    pub fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, SglangError> {
        self.transport
            .generate(self.endpoint(), &self.model, request)
    }

    pub fn stats(&self) -> Result<SglangStats, SglangError> {
        self.transport.stats(self.endpoint())
    }

    pub fn health(&self) -> Result<bool, SglangError> {
        self.transport.health(self.endpoint())
    }
}

/// Default config for SGLang.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::Sglang).with_endpoint(DEFAULT_ENDPOINT)
}
