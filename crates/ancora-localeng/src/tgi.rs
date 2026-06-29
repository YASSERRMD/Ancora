/// Integration with Hugging Face Text Generation Inference (TGI).
///
/// TGI exposes a generate endpoint and a health endpoint.  The default
/// port is 80 (Docker) or 8080 (local build).  This module wraps the
/// API with a typed client and mock transport.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Default TGI endpoint (local build).
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:8080";

/// Errors from the TGI client.
#[derive(Debug)]
pub enum TgiError {
    Unreachable(String),
    GenerationFailed(String),
    BadResponse(String),
}

impl std::fmt::Display for TgiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TgiError::Unreachable(u) => write!(f, "tgi unreachable: {}", u),
            TgiError::GenerationFailed(m) => write!(f, "generation failed: {}", m),
            TgiError::BadResponse(r) => write!(f, "bad response: {}", r),
        }
    }
}

/// TGI generation parameters.
#[derive(Debug, Clone)]
pub struct TgiParams {
    pub max_new_tokens: usize,
    pub temperature: f32,
    pub do_sample: bool,
    pub top_p: Option<f32>,
}

impl Default for TgiParams {
    fn default() -> Self {
        TgiParams {
            max_new_tokens: 256,
            temperature: 0.7,
            do_sample: true,
            top_p: None,
        }
    }
}

impl TgiParams {
    pub fn from_request(req: &CompletionRequest) -> Self {
        TgiParams {
            max_new_tokens: req.max_tokens,
            temperature: req.temperature,
            ..Default::default()
        }
    }
}

/// Info block returned by TGI /info endpoint.
#[derive(Debug, Clone)]
pub struct TgiInfo {
    pub model_id: String,
    pub max_input_length: usize,
    pub max_total_tokens: usize,
}

/// Transport abstraction for TGI.
pub trait TgiTransport {
    fn generate(
        &self,
        endpoint: &str,
        inputs: &str,
        params: &TgiParams,
    ) -> Result<CompletionResult, TgiError>;

    fn info(&self, endpoint: &str) -> Result<TgiInfo, TgiError>;
    fn health(&self, endpoint: &str) -> Result<bool, TgiError>;
}

/// TGI client.
pub struct TgiClient<T: TgiTransport> {
    pub config: EngineConfig,
    transport: T,
}

impl<T: TgiTransport> TgiClient<T> {
    pub fn new(config: EngineConfig, transport: T) -> Self {
        TgiClient { config, transport }
    }

    pub fn endpoint(&self) -> &str {
        self.config.endpoint.as_deref().unwrap_or(DEFAULT_ENDPOINT)
    }

    pub fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, TgiError> {
        let params = TgiParams::from_request(request);
        self.transport.generate(self.endpoint(), &request.prompt, &params)
    }

    pub fn info(&self) -> Result<TgiInfo, TgiError> {
        self.transport.info(self.endpoint())
    }

    pub fn health(&self) -> Result<bool, TgiError> {
        self.transport.health(self.endpoint())
    }
}

/// Default config for TGI.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::Tgi).with_endpoint(DEFAULT_ENDPOINT)
}
