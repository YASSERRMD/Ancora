/// Integration with vLLM (https://github.com/vllm-project/vllm).
///
/// vLLM presents an OpenAI-compatible HTTP endpoint.  This module provides
/// typed request/response wrappers and a mock transport for offline testing.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default vLLM endpoint.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8000";

/// Errors from the vLLM client.
#[derive(Debug)]
pub enum VllmError {
    Unreachable(String),
    InvalidModel(String),
    BadResponse(String),
    Oom,
}

impl std::fmt::Display for VllmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VllmError::Unreachable(u) => write!(f, "vllm unreachable: {}", u),
            VllmError::InvalidModel(m) => write!(f, "invalid model: {}", m),
            VllmError::BadResponse(r) => write!(f, "bad response: {}", r),
            VllmError::Oom => write!(f, "out of GPU memory"),
        }
    }
}

/// Sampling parameters for vLLM requests.
#[derive(Debug, Clone)]
pub struct SamplingParams {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: usize,
    pub presence_penalty: f32,
}

impl Default for SamplingParams {
    fn default() -> Self {
        SamplingParams {
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 256,
            presence_penalty: 0.0,
        }
    }
}

impl SamplingParams {
    pub fn from_request(req: &CompletionRequest) -> Self {
        SamplingParams {
            temperature: req.temperature,
            max_tokens: req.max_tokens,
            ..Default::default()
        }
    }
}

/// Transport abstraction for vLLM.
pub trait VllmTransport {
    fn complete(
        &self,
        endpoint: &str,
        model: &str,
        prompt: &str,
        params: &SamplingParams,
    ) -> Result<CompletionResult, VllmError>;

    fn models(&self, endpoint: &str) -> Result<Vec<String>, VllmError>;
    fn health(&self, endpoint: &str) -> Result<bool, VllmError>;
}

/// vLLM client.
pub struct VllmClient<T: VllmTransport> {
    pub config: EngineConfig,
    pub model: String,
    transport: T,
}

impl<T: VllmTransport> VllmClient<T> {
    pub fn new(config: EngineConfig, model: &str, transport: T) -> Self {
        VllmClient {
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
    ) -> Result<CompletionResult, VllmError> {
        let params = SamplingParams::from_request(request);
        self.transport
            .complete(self.endpoint(), &self.model, &request.prompt, &params)
    }

    pub fn list_models(&self) -> Result<Vec<String>, VllmError> {
        self.transport.models(self.endpoint())
    }

    pub fn health(&self) -> Result<bool, VllmError> {
        self.transport.health(self.endpoint())
    }
}

/// Default config for vLLM.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::Vllm).with_endpoint(DEFAULT_ENDPOINT)
}
