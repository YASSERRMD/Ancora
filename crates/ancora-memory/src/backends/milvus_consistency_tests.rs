/// Consistency level tests for the Milvus backend.
/// Verifies that each consistency level is correctly propagated into request bodies.
/// All offline.

#[cfg(test)]
mod milvus_consistency_tests {
    use crate::backends::milvus::*;

    #[test]
    fn strong_consistency_propagates_to_search() {
        let body = search_with_consistency_body(
            "docs", &[0.1f32; 4], 5, metric_type::COSINE, consistency::STRONG,
        );
        assert_eq!(body["consistencyLevel"], consistency::STRONG);
    }

    #[test]
    fn bounded_consistency_propagates_to_search() {
        let body = search_with_consistency_body(
            "docs", &[0.1f32; 4], 5, metric_type::COSINE, consistency::BOUNDED,
        );
        assert_eq!(body["consistencyLevel"], consistency::BOUNDED);
    }

    #[test]
    fn session_consistency_propagates_to_search() {
        let body = search_with_consistency_body(
            "docs", &[0.1f32; 4], 5, metric_type::COSINE, consistency::SESSION,
        );
        assert_eq!(body["consistencyLevel"], consistency::SESSION);
    }

    #[test]
    fn eventual_consistency_propagates_to_search() {
        let body = search_with_consistency_body(
            "docs", &[0.1f32; 4], 5, metric_type::COSINE, consistency::EVENTUALLY,
        );
        assert_eq!(body["consistencyLevel"], consistency::EVENTUALLY);
    }

    #[test]
    fn default_search_body_has_no_consistency_override() {
        let body = search_body("docs", &[0.1f32; 4], 5, metric_type::COSINE, &["payload"]);
        assert!(body["consistencyLevel"].is_null(), "no consistency level in default search");
    }

    #[test]
    fn consistency_does_not_affect_anns_field() {
        let body = search_with_consistency_body(
            "docs", &[0.1f32; 4], 5, metric_type::L2, consistency::STRONG,
        );
        assert_eq!(body["annsField"], "embedding");
    }
}
