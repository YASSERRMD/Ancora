//! Core model types for local inference engines.

/// The type of local inference engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EngineKind {
    LlamaCppServer,
    LlamaCppEmbedded,
    Ollama,
    Vllm,
    Sglang,
    LmStudio,
    Tgi,
    OnnxRuntime,
}

impl std::fmt::Display for EngineKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EngineKind::LlamaCppServer => "llama.cpp-server",
            EngineKind::LlamaCppEmbedded => "llama.cpp-embedded",
            EngineKind::Ollama => "ollama",
            EngineKind::Vllm => "vllm",
            EngineKind::Sglang => "sglang",
            EngineKind::LmStudio => "lm-studio",
            EngineKind::Tgi => "tgi",
            EngineKind::OnnxRuntime => "onnx-runtime",
        };
        write!(f, "{}", name)
    }
}

/// Configuration for connecting to or embedding a local engine.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub kind: EngineKind,
    pub endpoint: Option<String>,
    pub model_path: Option<String>,
    pub context_size: usize,
    pub threads: usize,
    pub gpu_layers: i32,
}

impl EngineConfig {
    pub fn new(kind: EngineKind) -> Self {
        EngineConfig {
            kind,
            endpoint: None,
            model_path: None,
            context_size: 2048,
            threads: 4,
            gpu_layers: 0,
        }
    }

    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.to_string());
        self
    }

    pub fn with_model_path(mut self, path: &str) -> Self {
        self.model_path = Some(path.to_string());
        self
    }

    pub fn with_context_size(mut self, size: usize) -> Self {
        self.context_size = size;
        self
    }

    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn with_gpu_layers(mut self, layers: i32) -> Self {
        self.gpu_layers = layers;
        self
    }
}

/// Result of a completion request.
#[derive(Debug, Clone)]
pub struct CompletionResult {
    pub text: String,
    pub tokens_generated: usize,
    pub engine: EngineKind,
}

/// A completion request payload.
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub stop_sequences: Vec<String>,
}

impl CompletionRequest {
    pub fn new(prompt: &str) -> Self {
        CompletionRequest {
            prompt: prompt.to_string(),
            max_tokens: 256,
            temperature: 0.7,
            stop_sequences: vec![],
        }
    }

    pub fn with_max_tokens(mut self, n: usize) -> Self {
        self.max_tokens = n;
        self
    }

    pub fn with_temperature(mut self, t: f32) -> Self {
        self.temperature = t;
        self
    }
}

#[cfg(test)]
mod model_tests {
    use super::*;

    #[test]
    fn engine_kind_display() {
        assert_eq!(EngineKind::Ollama.to_string(), "ollama");
        assert_eq!(EngineKind::LlamaCppServer.to_string(), "llama.cpp-server");
        assert_eq!(EngineKind::OnnxRuntime.to_string(), "onnx-runtime");
    }

    #[test]
    fn engine_config_builder() {
        let cfg = EngineConfig::new(EngineKind::Vllm)
            .with_endpoint("http://localhost:8000")
            .with_context_size(4096)
            .with_threads(8)
            .with_gpu_layers(32);
        assert_eq!(cfg.kind, EngineKind::Vllm);
        assert_eq!(cfg.endpoint.as_deref(), Some("http://localhost:8000"));
        assert_eq!(cfg.context_size, 4096);
        assert_eq!(cfg.threads, 8);
        assert_eq!(cfg.gpu_layers, 32);
    }

    #[test]
    fn completion_request_defaults() {
        let req = CompletionRequest::new("hello");
        assert_eq!(req.prompt, "hello");
        assert_eq!(req.max_tokens, 256);
        assert!((req.temperature - 0.7).abs() < 1e-6);
    }
}
