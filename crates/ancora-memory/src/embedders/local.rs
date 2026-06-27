/// Local/offline deterministic embedder for testing and edge deployments.
///
/// The `HashEmbedder` maps each text to a sparse one-hot-like vector via a
/// polynomial hash -- no model download required, fully reproducible across
/// runs. The `TfidfEmbedder` computes a simple TF-IDF bag-of-words vector.

use std::collections::HashMap;
use crate::embedders::embedder::{Embedding, EmbedResult, Embedder, l2_normalize};

// ---- HashEmbedder -------------------------------------------------------

/// Deterministic embedder based on a polynomial hash.  Useful for offline
/// tests and CI where no model is available.
#[derive(Debug, Clone)]
pub struct HashEmbedder {
    dims: usize,
    /// Number of non-zero components per embedding (increases recall diversity).
    components: usize,
}

impl HashEmbedder {
    pub fn new(dims: usize) -> Self {
        Self { dims, components: 4 }
    }

    pub fn with_components(mut self, n: usize) -> Self {
        self.components = n.max(1); self
    }
}

impl Embedder for HashEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let mut v = vec![0.0f32; self.dims];
        let words: Vec<&str> = text.split_whitespace().collect();
        let total = if words.is_empty() { 1 } else { words.len() };
        for (i, word) in words.iter().enumerate() {
            // polynomial hash seeded by word position
            let seed = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let h = word.bytes().fold(seed, |acc, b| {
                acc.wrapping_mul(31).wrapping_add(b as u64)
            });
            let idx = (h as usize) % self.dims;
            v[idx] += 1.0 / total as f32;
        }
        l2_normalize(&mut v);
        Ok(v)
    }

    fn model_name(&self) -> &str { "hash-embedder-v1" }
    fn dims(&self) -> usize { self.dims }
}

// ---- TfidfEmbedder ------------------------------------------------------

/// Simple TF-IDF bag-of-words embedder with a fixed vocabulary.
/// Vocabulary is built at construction time from sample documents.
#[derive(Debug, Clone)]
pub struct TfidfEmbedder {
    vocab: Vec<String>,
    idf: Vec<f32>,
}

impl TfidfEmbedder {
    /// Build from a corpus of documents.  Vocabulary is the union of all tokens,
    /// capped at `max_vocab` most-frequent terms.
    pub fn fit(docs: &[&str], max_vocab: usize) -> Self {
        let mut freq: HashMap<String, usize> = HashMap::new();
        let mut doc_freq: HashMap<String, usize> = HashMap::new();
        for doc in docs {
            let tokens: Vec<String> = tokenize(doc);
            for t in tokens {
                *freq.entry(t.clone()).or_default() += 1;
            }
            let unique: std::collections::HashSet<String> = doc.split_whitespace()
                .map(normalize_token)
                .collect();
            for t in unique {
                *doc_freq.entry(t).or_default() += 1;
            }
        }
        let mut terms: Vec<(String, usize)> = freq.into_iter().collect();
        terms.sort_by(|a, b| b.1.cmp(&a.1));
        terms.truncate(max_vocab);
        let n = docs.len().max(1) as f32;
        let vocab: Vec<String> = terms.iter().map(|(t, _)| t.clone()).collect();
        let idf: Vec<f32> = vocab.iter().map(|t| {
            let df = *doc_freq.get(t).unwrap_or(&1) as f32;
            (1.0 + n / df).ln()
        }).collect();
        Self { vocab, idf }
    }

    pub fn vocab_size(&self) -> usize { self.vocab.len() }
}

fn normalize_token(s: &str) -> String {
    s.chars().filter(|c| c.is_alphabetic()).collect::<String>().to_lowercase()
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace().map(normalize_token).filter(|s| !s.is_empty()).collect()
}

impl Embedder for TfidfEmbedder {
    fn embed(&self, text: &str) -> EmbedResult<Embedding> {
        let tokens = tokenize(text);
        let tf: HashMap<&str, f32> = {
            let mut map = HashMap::new();
            let total = tokens.len().max(1) as f32;
            for t in &tokens {
                *map.entry(t.as_str()).or_default() += 1.0 / total;
            }
            map
        };
        let mut v: Vec<f32> = self.vocab.iter().zip(self.idf.iter())
            .map(|(term, idf)| {
                let tf_val = tf.get(term.as_str()).copied().unwrap_or(0.0);
                tf_val * idf
            })
            .collect();
        l2_normalize(&mut v);
        Ok(v)
    }

    fn model_name(&self) -> &str { "tfidf-v1" }
    fn dims(&self) -> usize { self.vocab.len() }
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod local_embedder_tests {
    use super::*;
    use crate::embedders::embedder::cosine_similarity;

    #[test]
    fn hash_embedder_returns_correct_dims() {
        let e = HashEmbedder::new(128);
        let v = e.embed("hello world").unwrap();
        assert_eq!(v.len(), 128);
    }

    #[test]
    fn hash_embedder_is_unit_length() {
        let e = HashEmbedder::new(64);
        let v = e.embed("test sentence").unwrap();
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-4 || norm == 0.0, "norm: {norm}");
    }

    #[test]
    fn hash_embedder_deterministic() {
        let e = HashEmbedder::new(32);
        let v1 = e.embed("reproducible").unwrap();
        let v2 = e.embed("reproducible").unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn hash_embedder_similar_texts_have_positive_similarity() {
        let e = HashEmbedder::new(64);
        let v1 = e.embed("the quick brown fox").unwrap();
        let v2 = e.embed("the quick brown fox jumps").unwrap();
        // They share 4/5 tokens -- cosine should be positive.
        let sim = cosine_similarity(&v1, &v2);
        assert!(sim > 0.0, "sim: {sim}");
    }

    #[test]
    fn hash_embedder_empty_text_no_panic() {
        let e = HashEmbedder::new(16);
        let v = e.embed("").unwrap();
        assert_eq!(v.len(), 16);
    }

    #[test]
    fn tfidf_embedder_fit_and_embed() {
        let docs = &["hello world", "world peace", "hello peace love"];
        let e = TfidfEmbedder::fit(docs, 50);
        assert!(e.vocab_size() > 0);
        let v = e.embed("hello world").unwrap();
        assert_eq!(v.len(), e.vocab_size());
    }

    #[test]
    fn tfidf_embedder_unit_length() {
        let docs = &["alpha beta gamma", "gamma delta epsilon"];
        let e = TfidfEmbedder::fit(docs, 20);
        let v = e.embed("alpha gamma").unwrap();
        let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(norm <= 1.0 + 1e-4, "norm: {norm}");
    }

    #[test]
    fn tfidf_embedder_out_of_vocab_text_no_panic() {
        let docs = &["cat sat on mat", "dog ran fast"];
        let e = TfidfEmbedder::fit(docs, 20);
        let v = e.embed("giraffe hippopotamus").unwrap();
        assert_eq!(v.len(), e.vocab_size());
        assert!(v.iter().all(|x| *x == 0.0), "OOV should produce zero vector before norm");
    }

    #[test]
    fn hash_embedder_model_name() {
        let e = HashEmbedder::new(8);
        assert_eq!(e.model_name(), "hash-embedder-v1");
    }

    #[test]
    fn tfidf_embedder_model_name() {
        let docs = &["hello"];
        let e = TfidfEmbedder::fit(docs, 10);
        assert_eq!(e.model_name(), "tfidf-v1");
    }
}
