/// Qwen and GLM embedding endpoint helpers.
///
/// Both Qwen (via DashScope / Alibaba Cloud) and GLM (via Zhipu AI) expose
/// OpenAI-compatible `/v1/embeddings` endpoints. This module provides
/// config structs and request body helpers specialised for each provider.

use serde_json::{json, Value};
use crate::embedders::embedder::{Embedding, EmbedResult, Embedder};

// ---- Qwen ---------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct QwenEmbedConfig {
    /// DashScope base URL.
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub timeout_secs: u64,
}

impl QwenEmbedConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".to_owned(),
            api_key: api_key.into(),
            model: "text-embedding-v3".to_owned(),
            timeout_secs: 30,
        }
    }

    pub fn with_model(mut self, m: impl Into<String>) -> Self {
        self.model = m.into(); self
    }

    pub fn embeddings_url(&self) -> String {
        format!("{}/embeddings", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }
}

pub fn qwen_embed_body(config: &QwenEmbedConfig, texts: &[&str]) -> Value {
    json!({
        "model": config.model,
        "input": texts,
        "encoding_format": "float",
    })
}

// ---- GLM ----------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct GlmEmbedConfig {
    /// Zhipu AI base URL.
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub timeout_secs: u64,
}

impl GlmEmbedConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://open.bigmodel.cn/api/paas/v4".to_owned(),
            api_key: api_key.into(),
            model: "embedding-3".to_owned(),
            timeout_secs: 30,
        }
    }

    pub fn with_model(mut self, m: impl Into<String>) -> Self {
        self.model = m.into(); self
    }

    pub fn embeddings_url(&self) -> String {
        format!("{}/embeddings", self.base_url.trim_end_matches('/'))
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }
}

pub fn glm_embed_body(config: &GlmEmbedConfig, texts: &[&str]) -> Value {
    json!({
        "model": config.model,
        "input": texts,
    })
}

// ---- offline embedders --------------------------------------------------

/// Offline stub for Qwen embedder.
#[derive(Debug, Clone)]
pub struct QwenEmbedder {
    pub config: QwenEmbedConfig,
    pub dims: usize,
}

impl QwenEmbedder {
    pub fn new(config: QwenEmbedConfig, dims: usize) -> Self { Self { config, dims } }
}

impl Embedder for QwenEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let mut v = vec![0.0f32; self.dims];
        let h = text.bytes().fold(0xDEAD_BEEFu64, |acc, b| {
            acc.wrapping_mul(53).wrapping_add(b as u64)
        });
        v[(h as usize) % self.dims] = 1.0;
        Ok(v)
    }
    fn model_name(&self) -> &str { &self.config.model }
    fn dims(&self) -> usize { self.dims }
}

/// Offline stub for GLM embedder.
#[derive(Debug, Clone)]
pub struct GlmEmbedder {
    pub config: GlmEmbedConfig,
    pub dims: usize,
}

impl GlmEmbedder {
    pub fn new(config: GlmEmbedConfig, dims: usize) -> Self { Self { config, dims } }
}

impl Embedder for GlmEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let mut v = vec![0.0f32; self.dims];
        let h = text.bytes().fold(0xC0DE_CAFEu64, |acc, b| {
            acc.wrapping_mul(41).wrapping_add(b as u64)
        });
        v[(h as usize) % self.dims] = 1.0;
        Ok(v)
    }
    fn model_name(&self) -> &str { &self.config.model }
    fn dims(&self) -> usize { self.dims }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod qwen_glm_tests {
    use super::*;

    #[test]
    fn qwen_url_ends_with_embeddings() {
        let cfg = QwenEmbedConfig::new("key");
        assert!(cfg.embeddings_url().ends_with("/embeddings"), "url: {}", cfg.embeddings_url());
    }

    #[test]
    fn qwen_auth_header_is_bearer() {
        let cfg = QwenEmbedConfig::new("qwen-key");
        assert_eq!(cfg.auth_header(), "Bearer qwen-key");
    }

    #[test]
    fn qwen_body_model_propagates() {
        let cfg = QwenEmbedConfig::new("key");
        let body = qwen_embed_body(&cfg, &["hello"]);
        assert_eq!(body["model"], "text-embedding-v3");
    }

    #[test]
    fn qwen_body_input_array_length() {
        let cfg = QwenEmbedConfig::new("key");
        let body = qwen_embed_body(&cfg, &["a", "b"]);
        assert_eq!(body["input"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn glm_url_ends_with_embeddings() {
        let cfg = GlmEmbedConfig::new("key");
        assert!(cfg.embeddings_url().ends_with("/embeddings"), "url: {}", cfg.embeddings_url());
    }

    #[test]
    fn glm_auth_header_is_bearer() {
        let cfg = GlmEmbedConfig::new("glm-key");
        assert_eq!(cfg.auth_header(), "Bearer glm-key");
    }

    #[test]
    fn glm_body_model_propagates() {
        let cfg = GlmEmbedConfig::new("key");
        let body = glm_embed_body(&cfg, &["nihao"]);
        assert_eq!(body["model"], "embedding-3");
    }

    #[test]
    fn qwen_offline_embedder_dims() {
        let cfg = QwenEmbedConfig::new("key");
        let e = QwenEmbedder::new(cfg, 16);
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 16);
    }

    #[test]
    fn glm_offline_embedder_dims() {
        let cfg = GlmEmbedConfig::new("key");
        let e = GlmEmbedder::new(cfg, 8);
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 8);
    }

    #[test]
    fn qwen_and_glm_produce_different_vectors() {
        let qwen = QwenEmbedder::new(QwenEmbedConfig::new("k"), 16);
        let glm  = GlmEmbedder::new(GlmEmbedConfig::new("k"),  16);
        let v1 = qwen.embed("hello").unwrap();
        let v2 = glm.embed("hello").unwrap();
        // Different hash seeds -> different vectors for same text.
        assert_ne!(v1, v2, "different providers should produce different hash vectors");
    }
}
