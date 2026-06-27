/// Cohere embedding and reranking API helpers.
///
/// Covers `POST /v2/embed` for embeddings and `POST /v2/rerank` for reranking.
/// All helpers return descriptor `Value`s; no live HTTP calls are made here.

use serde_json::{json, Value};
use crate::embedders::embedder::{Embedding, EmbedError, EmbedResult, Embedder, Reranker};

// ---- input type constants -----------------------------------------------

pub mod input_type {
    pub const SEARCH_DOCUMENT: &str = "search_document";
    pub const SEARCH_QUERY: &str = "search_query";
    pub const CLASSIFICATION: &str = "classification";
    pub const CLUSTERING: &str = "clustering";
}

// ---- config --------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CohereConfig {
    pub api_key: String,
    pub model: String,
    pub rerank_model: String,
    pub timeout_secs: u64,
}

impl CohereConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: "embed-english-v3.0".to_owned(),
            rerank_model: "rerank-english-v3.0".to_owned(),
            timeout_secs: 30,
        }
    }

    pub fn with_model(mut self, m: impl Into<String>) -> Self {
        self.model = m.into(); self
    }

    pub fn auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    pub fn embed_url(&self) -> &'static str { "https://api.cohere.ai/v2/embed" }
    pub fn rerank_url(&self) -> &'static str { "https://api.cohere.ai/v2/rerank" }
}

// ---- request body helpers -----------------------------------------------

pub fn embed_body(config: &CohereConfig, texts: &[&str], input_type: &str) -> Value {
    json!({
        "model": config.model,
        "texts": texts,
        "input_type": input_type,
        "embedding_types": ["float"],
    })
}

pub fn rerank_body(config: &CohereConfig, query: &str, documents: &[&str], top_n: usize) -> Value {
    json!({
        "model": config.rerank_model,
        "query": query,
        "documents": documents,
        "top_n": top_n,
        "return_documents": true,
    })
}

// ---- response parsing ---------------------------------------------------

/// Parse Cohere embed response: `{ "embeddings": { "float": [[...], ...] } }`.
pub fn parse_cohere_embeddings(body: &Value) -> EmbedResult<Vec<Embedding>> {
    body["embeddings"]["float"]
        .as_array()
        .ok_or_else(|| EmbedError::ParseError("missing embeddings.float".to_owned()))
        .map(|arr| {
            arr.iter().map(|row| {
                row.as_array().unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                    .collect()
            }).collect()
        })
}

/// Parse Cohere rerank response: returns (index, relevance_score) pairs sorted by score.
pub fn parse_rerank_results(body: &Value) -> Vec<(usize, f32)> {
    body["results"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|r| {
            let idx = r["index"].as_u64().unwrap_or(0) as usize;
            let score = r["relevance_score"].as_f64().unwrap_or(0.0) as f32;
            (idx, score)
        })
        .collect()
}

// ---- offline embedder ---------------------------------------------------

/// Offline stub implementing the Embedder trait for Cohere.
/// Returns a deterministic hash-based vector without HTTP calls.
#[derive(Debug, Clone)]
pub struct CohereEmbedder {
    pub config: CohereConfig,
    pub dims: usize,
}

impl CohereEmbedder {
    pub fn new(config: CohereConfig, dims: usize) -> Self {
        Self { config, dims }
    }
}

impl Embedder for CohereEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let mut v = vec![0.0f32; self.dims];
        let h = text.bytes().fold(0x1234_5678u64, |acc, b| {
            acc.wrapping_mul(37).wrapping_add(b as u64)
        });
        v[(h as usize) % self.dims] = 1.0;
        Ok(v)
    }
    fn model_name(&self) -> &str { &self.config.model }
    fn dims(&self) -> usize { self.dims }
}

/// Offline stub implementing the Reranker trait for Cohere.
#[derive(Debug, Clone)]
pub struct CohereReranker {
    pub config: CohereConfig,
}

impl CohereReranker {
    pub fn new(config: CohereConfig) -> Self { Self { config } }
}

impl Reranker for CohereReranker {
    fn rerank(&self, _query: &str, passages: &[&str]) -> EmbedResult<Vec<f32>> {
        // Offline stub: return descending scores.
        Ok((0..passages.len()).map(|i| 1.0 / (i + 1) as f32).collect())
    }
    fn model_name(&self) -> &str { &self.config.rerank_model }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod cohere_tests {
    use super::*;

    #[test]
    fn config_embed_url() {
        let cfg = CohereConfig::new("key");
        assert_eq!(cfg.embed_url(), "https://api.cohere.ai/v2/embed");
    }

    #[test]
    fn config_auth_header_is_bearer() {
        let cfg = CohereConfig::new("co-test");
        assert_eq!(cfg.auth_header(), "Bearer co-test");
    }

    #[test]
    fn embed_body_model_propagates() {
        let cfg = CohereConfig::new("key");
        let body = embed_body(&cfg, &["hello"], input_type::SEARCH_DOCUMENT);
        assert_eq!(body["model"], "embed-english-v3.0");
    }

    #[test]
    fn embed_body_input_type_propagates() {
        let cfg = CohereConfig::new("key");
        let body = embed_body(&cfg, &["q"], input_type::SEARCH_QUERY);
        assert_eq!(body["input_type"], "search_query");
    }

    #[test]
    fn embed_body_texts_count() {
        let cfg = CohereConfig::new("key");
        let body = embed_body(&cfg, &["a", "b", "c"], input_type::SEARCH_DOCUMENT);
        assert_eq!(body["texts"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn rerank_body_top_n() {
        let cfg = CohereConfig::new("key");
        let body = rerank_body(&cfg, "query", &["doc1", "doc2"], 2);
        assert_eq!(body["top_n"], 2);
    }

    #[test]
    fn parse_cohere_embeddings_success() {
        let body = serde_json::json!({
            "embeddings": { "float": [[0.1, 0.2], [0.3, 0.4]] }
        });
        let embs = parse_cohere_embeddings(&body).unwrap();
        assert_eq!(embs.len(), 2);
        assert_eq!(embs[0].len(), 2);
    }

    #[test]
    fn parse_rerank_results_sorted_by_score() {
        let body = serde_json::json!({
            "results": [
                { "index": 0, "relevance_score": 0.9 },
                { "index": 1, "relevance_score": 0.6 },
            ]
        });
        let results = parse_rerank_results(&body);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 0);
        assert!((results[0].1 - 0.9f32).abs() < 1e-4);
    }

    #[test]
    fn offline_embedder_correct_dims() {
        let cfg = CohereConfig::new("key");
        let e = CohereEmbedder::new(cfg, 32);
        let v = e.embed("test").unwrap();
        assert_eq!(v.len(), 32);
    }

    #[test]
    fn offline_reranker_descending_scores() {
        let cfg = CohereConfig::new("key");
        let r = CohereReranker::new(cfg);
        let scores = r.rerank("q", &["a", "b", "c"]).unwrap();
        assert_eq!(scores.len(), 3);
        assert!(scores[0] > scores[1], "scores should be descending");
    }
}
