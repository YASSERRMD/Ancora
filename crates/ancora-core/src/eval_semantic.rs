use std::sync::Arc;

use crate::eval::EvalScorer;

/// A dense embedding vector.
pub type Embedding = Vec<f32>;

/// Produces embeddings for text.
pub trait EmbedFn: Send + Sync {
    fn embed(&self, text: &str) -> Embedding;
}

/// Cosine similarity between two equal-length vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 { 0.0 } else { dot / (na * nb) }
}

/// Semantic equivalence scorer using cosine similarity of embeddings.
pub struct SemanticEquivalenceScorer {
    embed: Arc<dyn EmbedFn>,
    threshold: f32,
}

impl SemanticEquivalenceScorer {
    pub fn new(embed: Arc<dyn EmbedFn>, threshold: f32) -> Self {
        Self { embed, threshold }
    }
}

impl EvalScorer for SemanticEquivalenceScorer {
    fn name(&self) -> &str { "semantic_equivalence" }

    fn score(&self, candidate: &str, expected: &str) -> f64 {
        let a = self.embed.embed(candidate);
        let b = self.embed.embed(expected);
        let sim = cosine_similarity(&a, &b);
        if sim >= self.threshold { sim as f64 } else { (sim / self.threshold) as f64 * 0.5 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct IdentityEmbed;
    impl EmbedFn for IdentityEmbed {
        fn embed(&self, text: &str) -> Embedding {
            text.bytes().map(|b| b as f32).collect()
        }
    }

    #[test]
    fn cosine_similarity_identical_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity(&v, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_orthogonal_vectors() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!((cosine_similarity(&a, &b)).abs() < 1e-6);
    }

    #[test]
    fn cosine_similarity_zero_vector_returns_zero() {
        let a = vec![0.0, 0.0];
        let b = vec![1.0, 1.0];
        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn semantic_scorer_identical_text_scores_high() {
        use std::sync::Arc;
        let scorer = SemanticEquivalenceScorer::new(Arc::new(IdentityEmbed), 0.9);
        let s = scorer.score("hello world", "hello world");
        assert!((s - 1.0).abs() < 1e-6);
    }

    #[test]
    fn semantic_scorer_name_is_semantic_equivalence() {
        use std::sync::Arc;
        let scorer = SemanticEquivalenceScorer::new(Arc::new(IdentityEmbed), 0.5);
        assert_eq!(scorer.name(), "semantic_equivalence");
    }

    #[test]
    fn cosine_similarity_partial_overlap() {
        let a = vec![1.0, 1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let s = cosine_similarity(&a, &b);
        assert!(s > 0.0 && s < 1.0);
    }
}
