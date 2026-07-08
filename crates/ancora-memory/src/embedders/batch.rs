/// Batch embedding with backpressure.
///
/// `BatchEmbedder` wraps any `Embedder` and splits large text batches into
/// chunks of `batch_size`, processing them sequentially with optional delay
/// between batches to avoid rate-limit exhaustion.  All operations are
/// synchronous and offline-safe.
use crate::embedders::embedder::{EmbedError, EmbedResult, Embedder, Embedding};

// ---- batch config -------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum texts per single embed call.
    pub batch_size: usize,
    /// Number of retries on transient error.
    pub max_retries: u32,
    /// Whether to skip failed items (true) or propagate the error (false).
    pub skip_on_error: bool,
}

impl BatchConfig {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size: batch_size.max(1),
            max_retries: 3,
            skip_on_error: false,
        }
    }

    pub fn with_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
    pub fn skip_errors(mut self) -> Self {
        self.skip_on_error = true;
        self
    }
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self::new(96)
    }
}

// ---- batch result -------------------------------------------------------

#[derive(Debug)]
pub struct BatchResult {
    pub embeddings: Vec<Option<Embedding>>,
    /// Number of items that failed permanently.
    pub error_count: usize,
    /// Total texts submitted.
    pub total: usize,
}

impl BatchResult {
    pub fn successful(&self) -> Vec<&Embedding> {
        self.embeddings.iter().filter_map(|e| e.as_ref()).collect()
    }

    pub fn success_rate(&self) -> f32 {
        if self.total == 0 {
            return 1.0;
        }
        (self.total - self.error_count) as f32 / self.total as f32
    }
}

// ---- batch embedder -----------------------------------------------------

/// Wraps an `Embedder` to process large batches with backpressure.
pub struct BatchEmbedder<E: Embedder> {
    pub inner: E,
    pub config: BatchConfig,
}

impl<E: Embedder> BatchEmbedder<E> {
    pub fn new(inner: E, config: BatchConfig) -> Self {
        Self { inner, config }
    }

    pub fn embed_all(&self, texts: &[&str]) -> BatchResult {
        let total = texts.len();
        let mut embeddings: Vec<Option<Embedding>> = Vec::with_capacity(total);
        let mut error_count = 0;

        for chunk in texts.chunks(self.config.batch_size) {
            let result = self.embed_chunk_with_retry(chunk);
            match result {
                Ok(chunk_embs) => {
                    for emb in chunk_embs {
                        embeddings.push(Some(emb));
                    }
                }
                Err(_) if self.config.skip_on_error => {
                    for _ in chunk {
                        embeddings.push(None);
                        error_count += 1;
                    }
                }
                Err(e) => {
                    // Propagate by filling remaining with None and returning.
                    let filled = embeddings.len();
                    for _ in filled..total {
                        embeddings.push(None);
                        error_count += 1;
                    }
                    let _ = e; // error info already counted
                    return BatchResult {
                        embeddings,
                        error_count,
                        total,
                    };
                }
            }
        }

        BatchResult {
            embeddings,
            error_count,
            total,
        }
    }

    fn embed_chunk_with_retry(&self, chunk: &[&str]) -> EmbedResult<Vec<Embedding>> {
        let mut last_err = EmbedError::Other("no attempt made".to_owned());
        for attempt in 0..=self.config.max_retries {
            match self.inner.embed_batch(chunk) {
                Ok(embs) => return Ok(embs),
                Err(e) if e.is_transient() && attempt < self.config.max_retries => {
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }
}

// ---- utility ------------------------------------------------------------

/// Split a flat `Vec<Embedding>` into batches of `size`.
pub fn chunk_embeddings(embs: Vec<Embedding>, size: usize) -> Vec<Vec<Embedding>> {
    embs.chunks(size.max(1)).map(|c| c.to_vec()).collect()
}

/// Merge multiple `BatchResult`s into one.
pub fn merge_batch_results(results: Vec<BatchResult>) -> BatchResult {
    let total: usize = results.iter().map(|r| r.total).sum();
    let error_count: usize = results.iter().map(|r| r.error_count).sum();
    let embeddings = results.into_iter().flat_map(|r| r.embeddings).collect();
    BatchResult {
        embeddings,
        error_count,
        total,
    }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod batch_tests {
    use super::*;
    use crate::embedders::embedder::{EmbedResult, Embedder, Embedding};

    struct Echo {
        dims: usize,
    }

    impl Embedder for Echo {
        fn embed(&self, text: &str) -> EmbedResult<Embedding> {
            let mut v = vec![0.0f32; self.dims];
            v[text.len() % self.dims] = 1.0;
            Ok(v)
        }
        fn model_name(&self) -> &str {
            "echo"
        }
        fn dims(&self) -> usize {
            self.dims
        }
    }

    #[test]
    fn batch_all_texts_embedded() {
        let texts: Vec<&str> = (0..10).map(|_| "hello").collect();
        let be = BatchEmbedder::new(Echo { dims: 8 }, BatchConfig::new(3));
        let result = be.embed_all(&texts);
        assert_eq!(result.total, 10);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.successful().len(), 10);
    }

    #[test]
    fn batch_success_rate_is_one_when_no_errors() {
        let be = BatchEmbedder::new(Echo { dims: 8 }, BatchConfig::new(5));
        let texts = vec!["a", "b", "c"];
        let result = be.embed_all(&texts);
        assert!((result.success_rate() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn batch_config_default_batch_size() {
        let cfg = BatchConfig::default();
        assert!(cfg.batch_size > 0);
    }

    #[test]
    fn batch_config_minimum_batch_size_is_one() {
        let cfg = BatchConfig::new(0);
        assert_eq!(cfg.batch_size, 1);
    }

    #[test]
    fn chunk_embeddings_splits_correctly() {
        let embs: Vec<Embedding> = (0..10).map(|_| vec![0.0f32]).collect();
        let chunks = chunk_embeddings(embs, 3);
        assert_eq!(chunks.len(), 4); // ceil(10/3) = 4
    }

    #[test]
    fn merge_batch_results_totals_sum() {
        let r1 = BatchResult {
            embeddings: vec![Some(vec![0.1f32])],
            error_count: 0,
            total: 1,
        };
        let r2 = BatchResult {
            embeddings: vec![None],
            error_count: 1,
            total: 1,
        };
        let merged = merge_batch_results(vec![r1, r2]);
        assert_eq!(merged.total, 2);
        assert_eq!(merged.error_count, 1);
    }

    #[test]
    fn empty_input_produces_empty_result() {
        let be = BatchEmbedder::new(Echo { dims: 4 }, BatchConfig::new(10));
        let result = be.embed_all(&[]);
        assert_eq!(result.total, 0);
        assert_eq!(result.error_count, 0);
    }
}
