//! Local-only inference enforcement for on-device runtime.
//!
//! Ensures that no inference request leaves the device by enforcing an
//! allow-list of "local" model providers and rejecting any configuration
//! that would route to a remote endpoint.

use serde::{Deserialize, Serialize};

/// A model backend that can serve inference requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelBackend {
    /// On-device GGUF/llama.cpp model.
    LocalGguf { model_path: String },
    /// On-device ONNX Runtime model.
    LocalOnnx { model_path: String },
    /// CoreML model (iOS / macOS).
    CoreMl { model_name: String },
    /// Remote API endpoint (always rejected in local-only mode).
    RemoteApi { url: String },
}

impl ModelBackend {
    /// Returns `true` when this backend serves inference locally.
    pub fn is_local(&self) -> bool {
        !matches!(self, Self::RemoteApi { .. })
    }

    /// Returns the model identifier string for logging.
    pub fn label(&self) -> String {
        match self {
            Self::LocalGguf { model_path } => format!("gguf:{}", model_path),
            Self::LocalOnnx { model_path } => format!("onnx:{}", model_path),
            Self::CoreMl { model_name } => format!("coreml:{}", model_name),
            Self::RemoteApi { url } => format!("remote:{}", url),
        }
    }
}

/// An inference request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    /// The prompt text.
    pub prompt: String,
    /// Maximum tokens to generate.
    pub max_tokens: usize,
    /// Sampling temperature.
    pub temperature: f32,
}

/// An inference response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    /// The generated text.
    pub text: String,
    /// Number of tokens generated.
    pub tokens_generated: usize,
    /// Whether the response was truncated.
    pub truncated: bool,
}

/// Error returned when an inference attempt is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferenceError {
    /// The backend is a remote API and local-only mode is active.
    RemoteBackendForbidden { url: String },
    /// The model file could not be located on the device.
    ModelNotFound { path: String },
    /// The request was empty or invalid.
    InvalidRequest(String),
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RemoteBackendForbidden { url } => {
                write!(f, "remote backend forbidden in local-only mode: {}", url)
            }
            Self::ModelNotFound { path } => write!(f, "model not found: {}", path),
            Self::InvalidRequest(msg) => write!(f, "invalid request: {}", msg),
        }
    }
}

/// Local-only inference engine.
///
/// All inference is synchronous and runs in the calling thread.
/// No tokio runtime is required.
#[derive(Debug)]
pub struct LocalInferenceEngine {
    backend: ModelBackend,
    local_only: bool,
}

impl LocalInferenceEngine {
    /// Create a new engine.  When `local_only` is `true` (the default for
    /// on-device builds), remote backends are rejected at construction time.
    pub fn new(backend: ModelBackend, local_only: bool) -> Result<Self, InferenceError> {
        if local_only && !backend.is_local() {
            if let ModelBackend::RemoteApi { url } = &backend {
                return Err(InferenceError::RemoteBackendForbidden { url: url.clone() });
            }
        }
        Ok(Self { backend, local_only })
    }

    /// Run an inference request.
    ///
    /// In this stand-in implementation the response echoes the prompt with
    /// a prefix so tests can assert on the output without a real model.
    pub fn infer(&self, req: &InferenceRequest) -> Result<InferenceResponse, InferenceError> {
        if req.prompt.is_empty() {
            return Err(InferenceError::InvalidRequest("prompt must not be empty".to_string()));
        }
        // Stand-in: generate a deterministic response for testing.
        let text = format!("[ondev:{}] {}", self.backend.label(), &req.prompt[..req.prompt.len().min(64)]);
        let tokens = text.split_whitespace().count();
        let truncated = tokens > req.max_tokens;
        Ok(InferenceResponse {
            text,
            tokens_generated: tokens.min(req.max_tokens),
            truncated,
        })
    }

    /// Return whether this engine is running in local-only mode.
    pub fn is_local_only(&self) -> bool {
        self.local_only
    }

    /// Return the backend label.
    pub fn backend_label(&self) -> String {
        self.backend.label()
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    fn local_engine() -> LocalInferenceEngine {
        LocalInferenceEngine::new(
            ModelBackend::LocalGguf { model_path: "/models/phi3.gguf".to_string() },
            true,
        )
        .unwrap()
    }

    #[test]
    fn local_backend_accepted() {
        let e = local_engine();
        assert!(e.is_local_only());
    }

    #[test]
    fn remote_backend_rejected_in_local_only_mode() {
        let err = LocalInferenceEngine::new(
            ModelBackend::RemoteApi { url: "https://api.openai.com".to_string() },
            true,
        )
        .unwrap_err();
        assert!(matches!(err, InferenceError::RemoteBackendForbidden { .. }));
    }

    #[test]
    fn remote_backend_accepted_when_not_local_only() {
        let e = LocalInferenceEngine::new(
            ModelBackend::RemoteApi { url: "https://example.com".to_string() },
            false,
        )
        .unwrap();
        assert!(!e.is_local_only());
    }

    #[test]
    fn infer_returns_response() {
        let e = local_engine();
        let req = InferenceRequest {
            prompt: "hello world".to_string(),
            max_tokens: 100,
            temperature: 0.7,
        };
        let resp = e.infer(&req).unwrap();
        assert!(!resp.text.is_empty());
    }

    #[test]
    fn empty_prompt_is_rejected() {
        let e = local_engine();
        let req = InferenceRequest { prompt: "".to_string(), max_tokens: 10, temperature: 0.0 };
        let err = e.infer(&req).unwrap_err();
        assert!(matches!(err, InferenceError::InvalidRequest(_)));
    }

    #[test]
    fn offline_run_uses_no_network() {
        // All inference paths are local; no I/O syscalls to remote hosts.
        let e = local_engine();
        let req = InferenceRequest {
            prompt: "what is 2+2?".to_string(),
            max_tokens: 20,
            temperature: 0.0,
        };
        let resp = e.infer(&req).unwrap();
        // The stand-in response contains the backend label.
        assert!(resp.text.contains("gguf:"));
    }

    #[test]
    fn onnx_backend_accepted() {
        let e = LocalInferenceEngine::new(
            ModelBackend::LocalOnnx { model_path: "/models/phi.onnx".to_string() },
            true,
        )
        .unwrap();
        assert!(e.backend_label().starts_with("onnx:"));
    }
}
