/// Integration with llama.cpp in embedded (in-process) mode.
///
/// Instead of an HTTP round-trip, the embedded integration links directly
/// against llama.cpp as a C library via FFI. For portability this module
/// provides a pure-Rust simulation layer (mock backend) so tests can run
/// without the native library present.
use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Errors from the embedded engine.
#[derive(Debug)]
pub enum EmbeddedError {
    ModelLoadFailed(String),
    InferenceFailed(String),
    InvalidConfig(String),
}

impl std::fmt::Display for EmbeddedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddedError::ModelLoadFailed(m) => write!(f, "model load failed: {}", m),
            EmbeddedError::InferenceFailed(m) => write!(f, "inference failed: {}", m),
            EmbeddedError::InvalidConfig(m) => write!(f, "invalid config: {}", m),
        }
    }
}

/// Parameters for initialising the embedded context.
#[derive(Debug, Clone)]
pub struct EmbeddedParams {
    pub n_ctx: usize,
    pub n_threads: usize,
    pub n_gpu_layers: i32,
    pub use_mlock: bool,
    pub use_mmap: bool,
}

impl Default for EmbeddedParams {
    fn default() -> Self {
        EmbeddedParams {
            n_ctx: 2048,
            n_threads: 4,
            n_gpu_layers: 0,
            use_mlock: false,
            use_mmap: true,
        }
    }
}

impl EmbeddedParams {
    pub fn from_config(cfg: &EngineConfig) -> Self {
        EmbeddedParams {
            n_ctx: cfg.context_size,
            n_threads: cfg.threads,
            n_gpu_layers: cfg.gpu_layers,
            ..Default::default()
        }
    }
}

/// Abstract backend so tests can inject a mock.
pub trait EmbeddedBackend {
    fn load_model(&mut self, path: &str, params: &EmbeddedParams) -> Result<(), EmbeddedError>;
    fn infer(&self, request: &CompletionRequest) -> Result<CompletionResult, EmbeddedError>;
    fn is_loaded(&self) -> bool;
}

/// A purely in-memory mock backend.
pub struct MockEmbeddedBackend {
    loaded: bool,
    model_path: Option<String>,
    fixed_response: String,
}

impl Default for MockEmbeddedBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MockEmbeddedBackend {
    pub fn new() -> Self {
        MockEmbeddedBackend {
            loaded: false,
            model_path: None,
            fixed_response: "mock embedded response".to_string(),
        }
    }

    pub fn with_fixed_response(mut self, response: &str) -> Self {
        self.fixed_response = response.to_string();
        self
    }
}

impl EmbeddedBackend for MockEmbeddedBackend {
    fn load_model(&mut self, path: &str, _params: &EmbeddedParams) -> Result<(), EmbeddedError> {
        if path.is_empty() {
            return Err(EmbeddedError::ModelLoadFailed("empty path".to_string()));
        }
        self.model_path = Some(path.to_string());
        self.loaded = true;
        Ok(())
    }

    fn infer(&self, request: &CompletionRequest) -> Result<CompletionResult, EmbeddedError> {
        if !self.loaded {
            return Err(EmbeddedError::InferenceFailed(
                "model not loaded".to_string(),
            ));
        }
        Ok(CompletionResult {
            text: format!("{} -> {}", request.prompt, self.fixed_response),
            tokens_generated: self.fixed_response.split_whitespace().count(),
            engine: EngineKind::LlamaCppEmbedded,
        })
    }

    fn is_loaded(&self) -> bool {
        self.loaded
    }
}

/// High-level embedded engine handle.
pub struct EmbeddedEngine<B: EmbeddedBackend> {
    pub config: EngineConfig,
    pub params: EmbeddedParams,
    backend: B,
}

impl<B: EmbeddedBackend> EmbeddedEngine<B> {
    pub fn new(config: EngineConfig, backend: B) -> Self {
        let params = EmbeddedParams::from_config(&config);
        EmbeddedEngine {
            config,
            params,
            backend,
        }
    }

    pub fn load(&mut self) -> Result<(), EmbeddedError> {
        let path = self
            .config
            .model_path
            .clone()
            .ok_or_else(|| EmbeddedError::InvalidConfig("model_path not set".to_string()))?;
        self.backend.load_model(&path, &self.params)
    }

    pub fn complete(&self, request: &CompletionRequest) -> Result<CompletionResult, EmbeddedError> {
        self.backend.infer(request)
    }

    pub fn is_ready(&self) -> bool {
        self.backend.is_loaded()
    }
}

/// Default config for embedded mode.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::LlamaCppEmbedded)
        .with_context_size(2048)
        .with_threads(4)
}
