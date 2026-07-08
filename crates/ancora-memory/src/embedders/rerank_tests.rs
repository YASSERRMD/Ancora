//! Extended reranker and RRF fusion tests -- all offline.

#[cfg(test)]
mod rerank_ext_tests {
    use crate::embedders::embedder::Reranker;
    use crate::embedders::rerank::*;

    // ---- RRF fusion extended tests -------------------------------------

    #[test]
    fn rrf_k_parameter_affects_score_magnitude() {
        // Smaller k -> higher scores for top-ranked items.
        let list = vec![0, 1, 2];
        let fused_k1 = reciprocal_rank_fusion(std::slice::from_ref(&list), 1.0);
        let fused_k60 = reciprocal_rank_fusion(&[list], 60.0);
        assert!(
            fused_k1[0].1 > fused_k60[0].1,
            "smaller k should give higher scores"
        );
    }

    #[test]
    fn rrf_three_lists_fusion() {
        let l1 = vec![0, 1, 2];
        let l2 = vec![1, 0, 2];
        let l3 = vec![0, 2, 1];
        let fused = reciprocal_rank_fusion(&[l1, l2, l3], 60.0);
        // doc 0: rank 1 in l1 and l3, rank 2 in l2 -> high score
        // doc 1: rank 2 in l1, rank 1 in l2, rank 3 in l3
        // Both should be top-2.
        let top_2: Vec<usize> = fused.iter().take(2).map(|(i, _)| *i).collect();
        assert!(top_2.contains(&0), "doc 0 should be top-2: {top_2:?}");
    }

    #[test]
    fn rrf_empty_lists_returns_empty() {
        let fused = reciprocal_rank_fusion(&[], 60.0);
        assert!(fused.is_empty());
    }

    #[test]
    fn rrf_single_item_list() {
        let fused = reciprocal_rank_fusion(&[vec![42usize]], 60.0);
        assert_eq!(fused.len(), 1);
        assert_eq!(fused[0].0, 42);
    }

    #[test]
    fn rrf_fuse_top_k_never_exceeds_total_items() {
        let dense = vec![0, 1, 2];
        let sparse = vec![0, 1, 2];
        let result = rrf_fuse(&dense, &sparse, 60.0, 10);
        assert!(result.len() <= 3, "result: {result:?}");
    }

    // ---- CosineReranker extended tests ---------------------------------

    #[test]
    fn cosine_reranker_empty_passage_embeddings() {
        let r = CosineReranker::new(vec![1.0f32, 0.0], vec![]);
        let scores = r.rerank("q", &[]).unwrap();
        assert!(scores.is_empty());
    }

    #[test]
    fn cosine_reranker_model_name() {
        let r = CosineReranker::new(vec![1.0f32], vec![]);
        assert_eq!(r.model_name(), "cosine-reranker");
    }

    #[test]
    fn cosine_reranker_score_sum_is_not_fixed() {
        // Scores can vary freely -- they are cosine similarities.
        let q = vec![1.0f32, 0.0];
        let embs = vec![vec![1.0f32, 0.0], vec![0.0f32, 1.0]];
        let r = CosineReranker::new(q, embs);
        let s = r.scores();
        assert!(
            s[0] != s[1],
            "distinct embeddings should give distinct scores"
        );
    }

    // ---- ScoredPassage -------------------------------------------------

    #[test]
    fn scored_passage_fields_accessible() {
        let p = ScoredPassage::new(7, "some passage text", 0.85);
        assert_eq!(p.index, 7);
        assert_eq!(p.text, "some passage text");
        assert!((p.score - 0.85f32).abs() < 1e-5);
    }

    // ---- IdentityReranker ----------------------------------------------

    #[test]
    fn identity_reranker_model_name() {
        let r = IdentityReranker;
        assert_eq!(r.model_name(), "identity");
    }

    #[test]
    fn identity_reranker_scores_are_positive() {
        let r = IdentityReranker;
        let scores = r.rerank("q", &["a", "b"]).unwrap();
        assert!(scores.iter().all(|s| *s > 0.0), "scores: {scores:?}");
    }

    // ---- sort_by_score -------------------------------------------------

    #[test]
    fn sort_by_score_empty_returns_empty() {
        let sorted = sort_by_score(vec![]);
        assert!(sorted.is_empty());
    }

    #[test]
    fn sort_by_score_single_element() {
        let p = ScoredPassage::new(0, "text", 0.5);
        let sorted = sort_by_score(vec![p]);
        assert_eq!(sorted.len(), 1);
    }

    #[test]
    fn sort_by_score_ties_preserved() {
        let p1 = ScoredPassage::new(0, "a", 0.5);
        let p2 = ScoredPassage::new(1, "b", 0.5);
        let sorted = sort_by_score(vec![p1, p2]);
        assert_eq!(sorted.len(), 2);
    }

    // ---- apply_rerank_scores -------------------------------------------

    #[test]
    fn apply_rerank_scores_returns_same_length() {
        let passages = vec![
            ScoredPassage::new(0, "a", 0.5),
            ScoredPassage::new(1, "b", 0.3),
        ];
        let scores = vec![0.9f32, 0.1f32];
        let updated = apply_rerank_scores(passages, scores);
        assert_eq!(updated.len(), 2);
    }
}
