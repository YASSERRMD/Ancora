/// Hybrid search and result merging tests for the Qdrant REST layer.
///
/// Tests RRF fusion body structure, score blending, dedup ordering, and
/// filter composition for hybrid queries. All offline.

#[cfg(test)]
mod qdrant_hybrid_tests {
    use crate::backends::qdrant::*;
    use crate::vector_store::{Filter, PayloadValue};

    #[test]
    fn rrf_fusion_with_one_vector_still_works() {
        let v = vec![0.1f32, 0.2, 0.3];
        let body = rrf_fusion_body(&[v.as_slice()], 5);
        assert_eq!(body["prefetch"].as_array().unwrap().len(), 1);
        assert_eq!(body["query"]["fusion"], "rrf");
    }

    #[test]
    fn rrf_fusion_limit_propagates_to_body() {
        let v = vec![0.1f32];
        let body = rrf_fusion_body(&[v.as_slice()], 42);
        assert_eq!(body["limit"], 42);
    }

    #[test]
    fn hybrid_query_body_limit_is_requested_top_k() {
        let body = hybrid_query_body(&[0.1f32], &[0], &[1.0], 10, 0.6);
        assert_eq!(body["limit"], 10);
    }

    #[test]
    fn hybrid_query_prefetch_limit_is_double_top_k() {
        let body = hybrid_query_body(&[0.1f32], &[0], &[1.0], 5, 0.6);
        let prefetch_limit = body["prefetch"][0]["limit"].as_u64().unwrap();
        assert_eq!(prefetch_limit, 10); // 5 * 2
    }

    #[test]
    fn sort_then_threshold_preserves_top_results() {
        let results = vec![
            (1u64, 0.95f32, serde_json::json!({})),
            (2, 0.60, serde_json::json!({})),
            (3, 0.82, serde_json::json!({})),
            (4, 0.40, serde_json::json!({})),
        ];
        let sorted = sort_by_score(results);
        let thresholded = apply_score_threshold(sorted, 0.75);
        assert_eq!(thresholded.len(), 2);
        assert_eq!(thresholded[0].0, 1);
        assert_eq!(thresholded[1].0, 3);
    }

    #[test]
    fn dedup_before_threshold_removes_duplicate_ids() {
        let results = vec![
            (1u64, 0.6f32, serde_json::json!({})),
            (1, 0.9, serde_json::json!({})), // duplicate, higher score wins
            (2, 0.5, serde_json::json!({})),
        ];
        let deduped = dedup_by_id(results);
        let thresholded = apply_score_threshold(deduped, 0.7);
        assert_eq!(thresholded.len(), 1);
        assert_eq!(thresholded[0].0, 1);
    }

    #[test]
    fn hybrid_query_with_filter_body_structure() {
        let f = Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned()));
        let dense = vec![0.1f32, 0.2];
        // hybrid_query_body does not take a filter directly -- filter would be
        // added as a top-level field in the query endpoint body by the caller.
        // We verify the search_body_with_threshold instead.
        let body = search_body_with_threshold(&dense, 5, 0.5, Some(&f));
        assert!(body["filter"].is_object());
        assert!((body["score_threshold"].as_f64().unwrap() - 0.5).abs() < 0.001);
    }

    #[test]
    fn named_vector_search_and_dedup_pipeline() {
        // Simulate results from two named-vector searches before dedup
        let text_results = vec![(1u64, 0.8f32, serde_json::json!({})), (2, 0.6, serde_json::json!({}))];
        let image_results = vec![(1u64, 0.7f32, serde_json::json!({})), (3, 0.9, serde_json::json!({}))];
        let mut combined = text_results;
        combined.extend(image_results);
        let mut deduped = dedup_by_id(combined);
        deduped = sort_by_score(deduped);
        assert_eq!(deduped[0].0, 3); // 0.9 from image
        assert_eq!(deduped[1].0, 1); // 0.8 from text (higher than 0.7 from image)
        assert_eq!(deduped.len(), 3);
    }
}
