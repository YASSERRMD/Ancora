/// Conformance tests for the Vespa backend with mock responses.
/// All offline.

#[cfg(test)]
mod vespa_conformance {
    use crate::backends::vespa::*;

    #[test]
    fn ann_query_includes_target_hits() {
        let q = ann_query("doc", 15, "embedding", "q_emb");
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("targetHits: 15"), "yql: {yql}");
    }

    #[test]
    fn bm25_query_ranking_profile() {
        let q = bm25_query("doc", "body", "search", 10);
        assert_eq!(q["ranking.profile"], "bm25");
    }

    #[test]
    fn hybrid_query_ranking_profile() {
        let q = hybrid_query("doc", 10, "emb", "q_emb", "test", 5, 0.5);
        assert_eq!(q["ranking.profile"], "hybrid");
    }

    #[test]
    fn put_document_body_fields_key_is_present() {
        let body = put_document_body(serde_json::json!({"title": "t"}));
        assert!(body["fields"].is_object());
    }

    #[test]
    fn update_document_uses_assign() {
        let body = update_document_body("year", serde_json::json!(2024), false);
        assert_eq!(body["fields"]["year"]["assign"], 2024);
    }

    #[test]
    fn yql_where_and_produces_valid_yql() {
        let result = yql_where_and("select * from doc where true", "year > 2020");
        assert!(result.contains("and (year > 2020)"), "result: {result}");
    }

    #[test]
    fn parse_hits_extracts_all_fields() {
        let body = serde_json::json!({
            "root": { "children": [
                { "id": "id:ns:doc::1", "relevance": 0.9, "fields": {"title": "x"} }
            ]}
        });
        let hits = parse_hits(&body);
        assert_eq!(hits.len(), 1);
        assert!((hits[0].1 - 0.9f32).abs() < 1e-4);
        assert_eq!(hits[0].2["title"], "x");
    }

    #[test]
    fn parse_total_count_from_response() {
        let body = serde_json::json!({ "root": { "fields": { "totalCount": 100 } } });
        assert_eq!(parse_total_count(&body), 100);
    }

    #[test]
    fn hybrid_ranking_profile_has_first_phase() {
        let p = hybrid_ranking_profile(0.5);
        assert!(p["first-phase"].is_object());
    }

    #[test]
    fn vespa_error_404_is_not_found() {
        let err = VespaError::from_response(404, "not found");
        assert!(matches!(err, VespaError::NotFound(_)));
    }
}
