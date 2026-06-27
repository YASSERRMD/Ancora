/// Core `Embedder` trait and associated types for the retrieval pipeline.
///
/// The trait is object-safe: it uses `&str` input and returns a `Vec<f32>`.
/// Implementations may wrap HTTP clients (feature-gated) or pure-Rust local
/// models.  All trait method signatures must remain `Send + Sync` so they can
/// be stored behind `Arc<dyn Embedder>`.

use serde_json::Value;

// ---- primary types -------------------------------------------------------

/// A dense floating-point embedding vector.
pub type Embedding = Vec<f32>;

/// Result type for embedding operations.
pub type EmbedResult<T> = Result<T, EmbedError>;

// ---- error type ---------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum EmbedError {
    /// The provider returned an HTTP error.
    HttpError { status: u16, body: String },
    /// The response JSON could not be parsed.
    ParseError(String),
    /// The input text was too long for the model context.
    InputTooLong { max_tokens: usize, got: usize },
    /// A transient error that may succeed on retry.
    Transient(String),
    /// Any other error.
    Other(String),
}

impl EmbedError {
    pub fn is_transient(&self) -> bool {
        matches!(self, Self::Transient(_) | Self::HttpError { status: 429 | 500..=599, .. })
    }
}

impl std::fmt::Display for EmbedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HttpError { status, body } => write!(f, "HTTP {status}: {body}"),
            Self::ParseError(msg) => write!(f, "parse error: {msg}"),
            Self::InputTooLong { max_tokens, got } => {
                write!(f, "input too long: max {max_tokens}, got {got}")
            }
            Self::Transient(msg) => write!(f, "transient: {msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

// ---- Embedder trait ------------------------------------------------------

/// An embedding provider that converts text into a dense vector.
///
/// Implementors may be backed by an HTTP API (OpenAI, Cohere, Qwen, GLM) or
/// a local deterministic model. Callers hold `Arc<dyn Embedder>`.
pub trait Embedder: Send + Sync {
    /// Embed a single text and return the dense vector.
    fn embed(&self, text: &str) -> EmbedResult<Embedding>;

    /// Embed a batch of texts.  Default implementation calls `embed` in a loop;
    /// providers with native batch endpoints should override this.
    fn embed_batch(&self, texts: &[&str]) -> EmbedResult<Vec<Embedding>> {
        texts.iter().map(|t| self.embed(t)).collect()
    }

    /// Name of the underlying model (e.g. `"text-embedding-3-small"`).
    fn model_name(&self) -> &str;

    /// Output dimensionality of the embedding vectors.
    fn dims(&self) -> usize;
}

// ---- Reranker trait ------------------------------------------------------

/// An optional reranker that scores (query, passage) pairs.
pub trait Reranker: Send + Sync {
    fn rerank(&self, query: &str, passages: &[&str]) -> EmbedResult<Vec<f32>>;
    fn model_name(&self) -> &str;
}

// ---- response parsing helpers -------------------------------------------

/// Parse the embedding vector from an OpenAI-style response body.
///
/// OpenAI format: `{ "data": [{ "embedding": [0.1, ...] }] }`
pub fn parse_openai_embedding(body: &Value) -> EmbedResult<Embedding> {
    body["data"][0]["embedding"]
        .as_array()
        .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect())
        .ok_or_else(|| EmbedError::ParseError("missing data[0].embedding".to_owned()))
}

/// Parse all embedding vectors from an OpenAI-style batch response.
pub fn parse_openai_batch_embeddings(body: &Value) -> EmbedResult<Vec<Embedding>> {
    body["data"]
        .as_array()
        .ok_or_else(|| EmbedError::ParseError("missing data array".to_owned()))
        .map(|arr| {
            arr.iter()
                .map(|item| {
                    item["embedding"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                        .collect()
                })
                .collect()
        })
}

/// Compute cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}

/// L2-normalize a vector in-place.
pub fn l2_normalize(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod embedder_trait_tests {
    use super::*;

    struct ConstantEmbedder {
        dim: usize,
    }

    impl Embedder for ConstantEmbedder {
        fn embed(&self, _text: &str) -> EmbedResult<Embedding> {
            Ok(vec![0.5f32; self.dim])
        }
        fn model_name(&self) -> &str { "constant-v1" }
        fn dims(&self) -> usize { self.dim }
    }

    #[test]
    fn embedder_trait_satisfied() {
        let e = ConstantEmbedder { dim: 4 };
        let v = e.embed("hello").unwrap();
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], 0.5);
    }

    #[test]
    fn embed_batch_default_implementation() {
        let e = ConstantEmbedder { dim: 3 };
        let results = e.embed_batch(&["a", "b", "c"]).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|v| v.len() == 3));
    }

    #[test]
    fn cosine_similarity_parallel_vectors() {
        let a = [1.0f32, 0.0, 0.0];
        let b = [1.0f32, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-5, "sim: {sim}");
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors() {
        let a = [1.0f32, 0.0];
        let b = [0.0f32, 1.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-5, "sim: {sim}");
    }

    #[test]
    fn l2_normalize_unit_length() {
        let mut v = vec![3.0f32, 4.0f32];
        l2_normalize(&mut v);
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5, "norm: {norm}");
    }

    #[test]
    fn l2_normalize_zero_vector_no_panic() {
        let mut v = vec![0.0f32, 0.0f32];
        l2_normalize(&mut v); // must not panic or divide by zero
        assert_eq!(v, [0.0, 0.0]);
    }

    #[test]
    fn embed_error_http_5xx_is_transient() {
        let e = EmbedError::HttpError { status: 503, body: "unavailable".to_owned() };
        assert!(e.is_transient());
    }

    #[test]
    fn embed_error_http_4xx_not_transient_except_429() {
        let e = EmbedError::HttpError { status: 404, body: "not found".to_owned() };
        assert!(!e.is_transient());
    }

    #[test]
    fn embed_error_429_is_transient() {
        let e = EmbedError::HttpError { status: 429, body: "rate limited".to_owned() };
        assert!(e.is_transient());
    }

    #[test]
    fn embed_error_input_too_long_not_transient() {
        let e = EmbedError::InputTooLong { max_tokens: 8192, got: 9000 };
        assert!(!e.is_transient());
    }

    #[test]
    fn parse_openai_embedding_succeeds() {
        let body = serde_json::json!({
            "data": [{ "embedding": [0.1, 0.2, 0.3] }]
        });
        let emb = parse_openai_embedding(&body).unwrap();
        assert_eq!(emb.len(), 3);
        assert!((emb[0] - 0.1f32).abs() < 1e-5);
    }

    #[test]
    fn parse_openai_batch_embeddings_succeeds() {
        let body = serde_json::json!({
            "data": [
                { "embedding": [0.1, 0.2] },
                { "embedding": [0.3, 0.4] },
            ]
        });
        let embs = parse_openai_batch_embeddings(&body).unwrap();
        assert_eq!(embs.len(), 2);
        assert_eq!(embs[1].len(), 2);
    }

    #[test]
    fn parse_openai_embedding_missing_data_returns_err() {
        let body = serde_json::json!({ "error": "no data" });
        assert!(parse_openai_embedding(&body).is_err());
    }
}
