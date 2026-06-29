/// Integration with ONNX Runtime for local inference.
///
/// ONNX Runtime can run models exported to the ONNX format.  This module
/// provides abstractions for session management, input tensor preparation,
/// and output decoding.  A mock session is provided for offline tests.

use crate::model::{CompletionRequest, CompletionResult, EngineConfig, EngineKind};

/// Errors from the ONNX integration.
#[derive(Debug)]
pub enum OnnxError {
    SessionCreateFailed(String),
    ModelLoadFailed(String),
    InferenceFailed(String),
    InvalidInput(String),
}

impl std::fmt::Display for OnnxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnnxError::SessionCreateFailed(m) => write!(f, "session create failed: {}", m),
            OnnxError::ModelLoadFailed(m) => write!(f, "model load failed: {}", m),
            OnnxError::InferenceFailed(m) => write!(f, "inference failed: {}", m),
            OnnxError::InvalidInput(m) => write!(f, "invalid input: {}", m),
        }
    }
}

/// Execution provider preference order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionProvider {
    Cuda,
    CoreML,
    DirectML,
    Cpu,
}

impl std::fmt::Display for ExecutionProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExecutionProvider::Cuda => "CUDA",
            ExecutionProvider::CoreML => "CoreML",
            ExecutionProvider::DirectML => "DirectML",
            ExecutionProvider::Cpu => "CPU",
        };
        write!(f, "{}", s)
    }
}

/// Session configuration.
#[derive(Debug, Clone)]
pub struct OnnxSessionConfig {
    pub model_path: String,
    pub providers: Vec<ExecutionProvider>,
    pub intra_op_threads: usize,
    pub inter_op_threads: usize,
}

impl OnnxSessionConfig {
    pub fn new(model_path: &str) -> Self {
        OnnxSessionConfig {
            model_path: model_path.to_string(),
            providers: vec![ExecutionProvider::Cpu],
            intra_op_threads: 4,
            inter_op_threads: 1,
        }
    }

    pub fn with_providers(mut self, providers: Vec<ExecutionProvider>) -> Self {
        self.providers = providers;
        self
    }
}

/// Abstract ONNX session.
pub trait OnnxSession {
    fn run(&self, input_ids: &[i64]) -> Result<Vec<f32>, OnnxError>;
    fn input_names(&self) -> Vec<String>;
    fn output_names(&self) -> Vec<String>;
}

/// Mock ONNX session for offline testing.
pub struct MockOnnxSession {
    pub model_path: String,
}

impl MockOnnxSession {
    pub fn new(model_path: &str) -> Self {
        MockOnnxSession {
            model_path: model_path.to_string(),
        }
    }
}

impl OnnxSession for MockOnnxSession {
    fn run(&self, input_ids: &[i64]) -> Result<Vec<f32>, OnnxError> {
        if input_ids.is_empty() {
            return Err(OnnxError::InvalidInput("empty input_ids".to_string()));
        }
        // Return fake logits
        Ok(vec![0.1_f32; 1000])
    }

    fn input_names(&self) -> Vec<String> {
        vec!["input_ids".to_string(), "attention_mask".to_string()]
    }

    fn output_names(&self) -> Vec<String> {
        vec!["logits".to_string()]
    }
}

/// High-level ONNX inference engine.
pub struct OnnxEngine<S: OnnxSession> {
    pub config: EngineConfig,
    pub session_config: OnnxSessionConfig,
    session: S,
}

impl<S: OnnxSession> OnnxEngine<S> {
    pub fn new(config: EngineConfig, session_config: OnnxSessionConfig, session: S) -> Self {
        OnnxEngine {
            config,
            session_config,
            session,
        }
    }

    /// Tokenize a prompt into fake token IDs for mock usage.
    fn tokenize(prompt: &str) -> Vec<i64> {
        prompt
            .split_whitespace()
            .enumerate()
            .map(|(i, _)| i as i64 + 1)
            .collect()
    }

    /// Decode logits into a simple text response.
    fn decode_logits(_logits: &[f32]) -> String {
        "onnx decoded response".to_string()
    }

    pub fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResult, OnnxError> {
        let input_ids = Self::tokenize(&request.prompt);
        let logits = self.session.run(&input_ids)?;
        let text = Self::decode_logits(&logits);
        Ok(CompletionResult {
            text,
            tokens_generated: request.max_tokens.min(50),
            engine: EngineKind::OnnxRuntime,
        })
    }

    pub fn input_names(&self) -> Vec<String> {
        self.session.input_names()
    }
}

/// Default config for ONNX Runtime.
pub fn default_config() -> EngineConfig {
    EngineConfig::new(EngineKind::OnnxRuntime)
}
