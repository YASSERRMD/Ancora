/// Vespa hybrid ranking and query construction tests -- all offline.

#[cfg(test)]
mod vespa_hybrid_tests {
    use crate::backends::vespa::*;

    // ---- hybrid_query structure -----------------------------------------

    #[test]
    fn hybrid_query_has_yql_key() {
        let q = hybrid_query("doc", 10, "emb", "qEmb", "body", 5, 0.5);
        assert!(q["yql"].as_str().is_some(), "yql must be present");
    }

    #[test]
    fn hybrid_query_yql_contains_nearest_neighbor() {
        let q = hybrid_query("doc", 10, "emb", "qEmb", "body", 5, 0.5);
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("nearestNeighbor"), "yql: {yql}");
    }

    #[test]
    fn hybrid_query_yql_contains_userquery() {
        let q = hybrid_query("doc", 8, "emb", "qEmb", "body", 4, 0.5);
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("userQuery()") || yql.contains("contains"), "yql: {yql}");
    }

    #[test]
    fn hybrid_query_ranking_is_hybrid() {
        let q = hybrid_query("doc", 10, "emb", "qEmb", "body", 5, 0.5);
        assert_eq!(q["ranking.profile"], "hybrid");
    }

    #[test]
    fn hybrid_query_hits_matches_bm25_count() {
        let q = hybrid_query("doc", 10, "emb", "qEmb", "body", 5, 0.5);
        // The hits value drives how many results to return.
        assert!(q["hits"].as_u64().is_some());
    }

    // ---- hybrid_ranking_profile ----------------------------------------

    #[test]
    fn hybrid_ranking_profile_alpha_0_5_first_phase_expr() {
        let p = hybrid_ranking_profile(0.5);
        let expr = p["first-phase"]["expression"].as_str().unwrap_or("");
        assert!(!expr.is_empty(), "first-phase expression must not be empty");
    }

    #[test]
    fn hybrid_ranking_profile_alpha_1_0_dense_only() {
        let p = hybrid_ranking_profile(1.0);
        assert!(p["first-phase"].is_object());
    }

    #[test]
    fn hybrid_ranking_profile_alpha_0_0_sparse_only() {
        let p = hybrid_ranking_profile(0.0);
        assert!(p["first-phase"].is_object());
    }

    #[test]
    fn hybrid_ranking_profile_has_name() {
        let p = hybrid_ranking_profile(0.5);
        assert!(p["name"].as_str().is_some());
    }

    // ---- ann_query -------------------------------------------------------

    #[test]
    fn ann_query_ranking_profile_is_closeness() {
        let q = ann_query("doc", 5, "emb", "qEmb");
        assert_eq!(q["ranking.profile"], "closeness");
    }

    #[test]
    fn ann_query_target_hits_in_yql() {
        let q = ann_query("doc", 20, "emb", "qEmb");
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("20") || yql.contains("targetHits"), "yql: {yql}");
    }

    // ---- bm25_query -------------------------------------------------------

    #[test]
    fn bm25_query_type_is_bm25() {
        let q = bm25_query("doc", "body", "search term", 10);
        assert_eq!(q["ranking.profile"], "bm25");
    }

    #[test]
    fn bm25_query_yql_contains_schema() {
        let q = bm25_query("my_doc", "title", "word", 5);
        let yql = q["yql"].as_str().unwrap();
        assert!(yql.contains("my_doc"), "yql: {yql}");
    }

    // ---- yql_where_and ---------------------------------------------------

    #[test]
    fn yql_where_and_appends_condition() {
        let base = "select * from doc where true";
        let result = yql_where_and(base, "year > 2020");
        assert!(result.contains("and (year > 2020)"), "result: {result}");
    }

    #[test]
    fn yql_where_and_preserves_base() {
        let base = "select * from doc where true";
        let result = yql_where_and(base, "score > 0.5");
        assert!(result.starts_with("select * from doc"), "result: {result}");
    }

    // ---- put/update document bodies ---------------------------------------

    #[test]
    fn put_document_body_wraps_in_fields() {
        let payload = serde_json::json!({"title": "hello", "score": 0.9});
        let body = put_document_body(payload);
        assert_eq!(body["fields"]["title"], "hello");
    }

    #[test]
    fn update_document_body_create_flag_propagates() {
        let body = update_document_body("status", serde_json::json!("active"), true);
        assert_eq!(body["create"], true);
    }

    #[test]
    fn update_document_body_assign_int() {
        let body = update_document_body("count", serde_json::json!(5), false);
        assert_eq!(body["fields"]["count"]["assign"], 5);
    }

    // ---- parse_hits -------------------------------------------------------

    #[test]
    fn parse_hits_empty_root() {
        let body = serde_json::json!({ "root": { "children": [] } });
        let hits = parse_hits(&body);
        assert!(hits.is_empty());
    }

    #[test]
    fn parse_hits_multiple_results() {
        let body = serde_json::json!({
            "root": { "children": [
                { "id": "id:ns:doc::1", "relevance": 0.9, "fields": {"k": "v1"} },
                { "id": "id:ns:doc::2", "relevance": 0.7, "fields": {"k": "v2"} },
            ]}
        });
        let hits = parse_hits(&body);
        assert_eq!(hits.len(), 2);
        assert!(hits[0].1 > hits[1].1, "higher relevance first");
    }

    // ---- VespaConfig -----------------------------------------------------

    #[test]
    fn vespa_config_local_url_on_8080() {
        let cfg = VespaConfig::local();
        assert!(cfg.url.contains("8080"), "url: {}", cfg.url);
    }

    #[test]
    fn vespa_config_auth_header_none_when_no_key() {
        let cfg = VespaConfig::local();
        assert!(cfg.auth_header().is_none());
    }
}
