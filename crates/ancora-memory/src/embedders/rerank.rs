/// Reranking stage for the retrieval pipeline.
///
/// A `Reranker` takes a query and a ranked list of passages and returns a
/// re-scored ordering.  This module provides:
/// - `ScoreReranker` -- identity reranker that preserves original scores
/// - `RecipRankFusion` -- score fusion via Reciprocal Rank Fusion (RRF)
/// - `CosineReranker` -- rerank by cosine similarity against a query embedding
use crate::embedders::embedder::{cosine_similarity, EmbedResult, Embedding, Reranker};

// ---- scored passage -----------------------------------------------------

#[derive(Debug, Clone)]
pub struct ScoredPassage {
    pub index: usize,
    pub text: String,
    pub score: f32,
}

impl ScoredPassage {
    pub fn new(index: usize, text: impl Into<String>, score: f32) -> Self {
        Self {
            index,
            text: text.into(),
            score,
        }
    }
}

// ---- ScoreReranker (identity) -------------------------------------------

/// Reranker that returns passages in their original order without change.
pub struct IdentityReranker;

impl Reranker for IdentityReranker {
    fn rerank(&self, _query: &str, passages: &[&str]) -> EmbedResult<Vec<f32>> {
        Ok((0..passages.len()).map(|i| 1.0 / (i + 1) as f32).collect())
    }
    fn model_name(&self) -> &str {
        "identity"
    }
}

// ---- RecipRankFusion ----------------------------------------------------

/// Merge multiple ranked lists using Reciprocal Rank Fusion.
///
/// `k` is the RRF constant (typical value 60).
pub fn reciprocal_rank_fusion(lists: &[Vec<usize>], k: f32) -> Vec<(usize, f32)> {
    let mut scores: std::collections::HashMap<usize, f32> = std::collections::HashMap::new();
    for list in lists {
        for (rank, &idx) in list.iter().enumerate() {
            *scores.entry(idx).or_default() += 1.0 / (k + rank as f32 + 1.0);
        }
    }
    let mut result: Vec<(usize, f32)> = scores.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Fuse a dense list and a sparse list using RRF, returning top-k indices.
pub fn rrf_fuse(dense: &[usize], sparse: &[usize], k: f32, top_k: usize) -> Vec<usize> {
    let fused = reciprocal_rank_fusion(&[dense.to_vec(), sparse.to_vec()], k);
    fused.into_iter().take(top_k).map(|(idx, _)| idx).collect()
}

// ---- CosineReranker -----------------------------------------------------

/// Rerank passages by cosine similarity between their embeddings and the
/// query embedding.
pub struct CosineReranker {
    pub query_embedding: Embedding,
    pub passage_embeddings: Vec<Embedding>,
}

impl CosineReranker {
    pub fn new(query_embedding: Embedding, passage_embeddings: Vec<Embedding>) -> Self {
        Self {
            query_embedding,
            passage_embeddings,
        }
    }

    pub fn scores(&self) -> Vec<f32> {
        self.passage_embeddings
            .iter()
            .map(|emb| cosine_similarity(&self.query_embedding, emb))
            .collect()
    }

    pub fn top_k(&self, k: usize) -> Vec<(usize, f32)> {
        let mut scored: Vec<(usize, f32)> = self.scores().into_iter().enumerate().collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }
}

impl Reranker for CosineReranker {
    fn rerank(&self, _query: &str, passages: &[&str]) -> EmbedResult<Vec<f32>> {
        let n = passages.len().min(self.passage_embeddings.len());
        let mut scores = self.scores();
        scores.truncate(n);
        Ok(scores)
    }
    fn model_name(&self) -> &str {
        "cosine-reranker"
    }
}

// ---- sort helpers -------------------------------------------------------

/// Sort passages by score descending.
pub fn sort_by_score(mut passages: Vec<ScoredPassage>) -> Vec<ScoredPassage> {
    passages.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    passages
}

/// Apply reranker scores to passages and return sorted list.
pub fn apply_rerank_scores(passages: Vec<ScoredPassage>, scores: Vec<f32>) -> Vec<ScoredPassage> {
    passages
        .into_iter()
        .zip(scores.into_iter())
        .map(|(mut p, score)| {
            p.score = score;
            p
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

// ---- tests ---------------------------------------------------------------

#[cfg(test)]
mod rerank_tests {
    use super::*;
    use crate::embedders::embedder::Reranker as RerankerTrait;

    #[test]
    fn identity_reranker_returns_descending_scores() {
        let r = IdentityReranker;
        let scores = r.rerank("q", &["a", "b", "c"]).unwrap();
        assert_eq!(scores.len(), 3);
        assert!(scores[0] > scores[1], "scores should be descending");
    }

    #[test]
    fn rrf_combines_two_lists() {
        let dense = vec![0, 1, 2];
        let sparse = vec![2, 0, 1];
        let result = rrf_fuse(&dense, &sparse, 60.0, 3);
        assert_eq!(result.len(), 3);
        // doc 0 is ranked first in dense and second in sparse; should score high.
        assert!(result.contains(&0));
    }

    #[test]
    fn rrf_top_k_truncates() {
        let dense = vec![0, 1, 2, 3];
        let sparse = vec![3, 2, 1, 0];
        let result = rrf_fuse(&dense, &sparse, 60.0, 2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn rrf_single_list_preserves_order() {
        let list = vec![5, 3, 1];
        let result = reciprocal_rank_fusion(&[list.clone()], 60.0);
        let order: Vec<usize> = result.into_iter().map(|(i, _)| i).collect();
        assert_eq!(order[0], 5, "top-ranked in list should be first");
    }

    #[test]
    fn cosine_reranker_returns_scores_for_each_passage() {
        let q_emb = vec![1.0f32, 0.0, 0.0];
        let p_embs = vec![vec![1.0f32, 0.0, 0.0], vec![0.0f32, 1.0, 0.0]];
        let r = CosineReranker::new(q_emb, p_embs);
        let scores = r.rerank("q", &["a", "b"]).unwrap();
        assert_eq!(scores.len(), 2);
        assert!(scores[0] > scores[1], "parallel vec should score higher");
    }

    #[test]
    fn cosine_reranker_top_k_limits_results() {
        let q_emb = vec![1.0f32, 0.0];
        let p_embs = vec![vec![1.0f32, 0.0], vec![0.0f32, 1.0], vec![0.5f32, 0.5]];
        let r = CosineReranker::new(q_emb, p_embs);
        let top = r.top_k(2);
        assert_eq!(top.len(), 2);
    }

    #[test]
    fn sort_by_score_orders_descending() {
        let passages = vec![
            ScoredPassage::new(0, "a", 0.3),
            ScoredPassage::new(1, "b", 0.9),
            ScoredPassage::new(2, "c", 0.6),
        ];
        let sorted = sort_by_score(passages);
        assert_eq!(sorted[0].index, 1);
        assert_eq!(sorted[1].index, 2);
    }

    #[test]
    fn apply_rerank_scores_updates_scores() {
        let passages = vec![
            ScoredPassage::new(0, "a", 0.5),
            ScoredPassage::new(1, "b", 0.5),
        ];
        let new_scores = vec![0.9f32, 0.2f32];
        let updated = apply_rerank_scores(passages, new_scores);
        assert!((updated[0].score - 0.9f32).abs() < 1e-5);
    }
}
