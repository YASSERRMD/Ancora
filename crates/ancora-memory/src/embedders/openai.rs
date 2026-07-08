use crate::embedders::embedder::{EmbedResult, Embedder, Embedding};
/// OpenAI-compatible embedding API helpers.
///
/// Works with OpenAI `/v1/embeddings` and any compatible endpoint
/// (Azure OpenAI, LiteLLM, Ollama with OpenAI shim, etc.).
/// Requires the `ureq` optional dep via any backend feature, or call via
/// your own HTTP client by using the descriptor returned by `request_body`.
use serde_json::{json, Value};

// ---- config --------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct OpenAiEmbedConfig {
    /// Base URL, e.g. `https://api.openai.com/v1`.
    pub base_url: String,
    /// API key (Bearer token).
    pub api_key: String,
    /// Model name, e.g. `"text-embedding-3-small"`.
    pub model: String,
    /// Output dimensions (None = use model default).
    pub dimensions: Option<usize>,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
}

impl OpenAiEmbedConfig {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_owned(),
            api_key: api_key.into(),
            model: model.into(),
            dimensions: None,
            timeout_secs: 30,
        }
    }

    /// Build config for Azure OpenAI.
    pub fn azure(
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        deployment: impl Into<String>,
    ) -> Self {
        let ep: String = endpoint.into();
        let dep: String = deployment.into();
        Self {
            base_url: format!("{ep}/openai/deployments/{dep}"),
            api_key: api_key.into(),
            model: dep,
            dimensions: None,
            timeout_secs: 30,
        }
    }

    /// Local compatible endpoint (Ollama, LiteLLM, etc.).
    pub fn local(model: impl Into<String>) -> Self {
        Self {
            base_url: "http://localhost:11434/v1".to_owned(),
            api_key: String::new(),
            model: model.into(),
            dimensions: None,
            timeout_secs: 60,
        }
    }

    pub fn with_dimensions(mut self, dims: usize) -> Self {
        self.dimensions = Some(dims);
        self
    }

    pub fn embeddings_url(&self) -> String {
        format!("{}/embeddings", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }
}

// ---- request body helpers -----------------------------------------------

/// Build a single-text embedding request body.
pub fn request_body(config: &OpenAiEmbedConfig, text: &str) -> Value {
    let mut body = json!({
        "model": config.model,
        "input": text,
        "encoding_format": "float",
    });
    if let Some(d) = config.dimensions {
        body["dimensions"] = json!(d);
    }
    body
}

/// Build a batch embedding request body.
pub fn batch_request_body(config: &OpenAiEmbedConfig, texts: &[&str]) -> Value {
    let mut body = json!({
        "model": config.model,
        "input": texts,
        "encoding_format": "float",
    });
    if let Some(d) = config.dimensions {
        body["dimensions"] = json!(d);
    }
    body
}

// ---- offline embedder ---------------------------------------------------

/// An offline stub for the OpenAI-compatible embedder that returns
/// deterministic vectors for use in tests without a live API.
#[derive(Debug, Clone)]
pub struct OpenAiEmbedder {
    pub config: OpenAiEmbedConfig,
    /// Fallback dims when config.dimensions is None.
    pub fallback_dims: usize,
}

impl OpenAiEmbedder {
    pub fn new(config: OpenAiEmbedConfig, fallback_dims: usize) -> Self {
        Self {
            config,
            fallback_dims,
        }
    }

    fn effective_dims(&self) -> usize {
        self.config.dimensions.unwrap_or(self.fallback_dims)
    }
}

impl Embedder for OpenAiEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let dims = self.effective_dims();
        let mut v = vec![0.0f32; dims];
        let h = text
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let idx = (h as usize) % dims;
        v[idx] = 1.0;
        Ok(v)
    }

    fn embed_batch(&self, texts: &[&str]) -> EmbedResult<Vec<Embedding>> {
        texts.iter().map(|t| self.embed(t)).collect()
    }

    fn model_name(&self) -> &str {
        &self.config.model
    }

    fn dims(&self) -> usize {
        self.effective_dims()
    }
}

// ---- response parsing ---------------------------------------------------

/// Parse usage info from OpenAI embedding response.
pub fn parse_usage(body: &Value) -> (u64, u64) {
    let prompt = body["usage"]["prompt_tokens"].as_u64().unwrap_or(0);
    let total = body["usage"]["total_tokens"].as_u64().unwrap_or(0);
    (prompt, total)
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod openai_tests {
    use super::*;

    #[test]
    fn config_embeddings_url_ends_with_embeddings() {
        let cfg = OpenAiEmbedConfig::new("key", "text-embedding-3-small");
        assert!(
            cfg.embeddings_url().ends_with("/embeddings"),
            "url: {}",
            cfg.embeddings_url()
        );
    }

    #[test]
    fn config_auth_header_is_bearer() {
        let cfg = OpenAiEmbedConfig::new("sk-test", "model");
        assert_eq!(cfg.auth_header(), "Bearer sk-test");
    }

    #[test]
    fn azure_config_url_contains_deployment() {
        let cfg = OpenAiEmbedConfig::azure("https://my.openai.azure.com", "key", "embed-dep");
        assert!(cfg.base_url.contains("embed-dep"), "url: {}", cfg.base_url);
    }

    #[test]
    fn request_body_contains_model() {
        let cfg = OpenAiEmbedConfig::new("key", "text-embedding-3-small");
        let body = request_body(&cfg, "hello");
        assert_eq!(body["model"], "text-embedding-3-small");
    }

    #[test]
    fn request_body_with_dimensions() {
        let cfg = OpenAiEmbedConfig::new("key", "text-embedding-3-small").with_dimensions(256);
        let body = request_body(&cfg, "hello");
        assert_eq!(body["dimensions"], 256);
    }

    #[test]
    fn batch_request_body_input_array() {
        let cfg = OpenAiEmbedConfig::new("key", "model");
        let body = batch_request_body(&cfg, &["a", "b", "c"]);
        assert_eq!(body["input"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn offline_embedder_returns_correct_dims() {
        let cfg = OpenAiEmbedConfig::new("key", "model").with_dimensions(16);
        let e = OpenAiEmbedder::new(cfg, 16);
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 16);
    }

    #[test]
    fn offline_embedder_deterministic() {
        let cfg = OpenAiEmbedConfig::new("key", "model").with_dimensions(8);
        let e = OpenAiEmbedder::new(cfg, 8);
        let v1 = e.embed("hello").unwrap();
        let v2 = e.embed("hello").unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn offline_embedder_different_for_very_different_texts() {
        // Use a large dimension space to make collisions very unlikely.
        let cfg = OpenAiEmbedConfig::new("key", "model").with_dimensions(1024);
        let e = OpenAiEmbedder::new(cfg, 1024);
        let v1 = e.embed("hello").unwrap();
        let v2 = e.embed("this_is_a_very_different_string_xyz_9876").unwrap();
        assert_ne!(
            v1, v2,
            "different texts should produce different embeddings"
        );
    }

    #[test]
    fn parse_usage_extracts_tokens() {
        let body = serde_json::json!({
            "usage": { "prompt_tokens": 10, "total_tokens": 10 }
        });
        let (prompt, total) = parse_usage(&body);
        assert_eq!(prompt, 10);
        assert_eq!(total, 10);
    }
}
