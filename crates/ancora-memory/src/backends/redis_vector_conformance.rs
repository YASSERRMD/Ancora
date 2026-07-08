/// Conformance tests for the Redis Vector (RediSearch) backend.
/// All offline -- no live Redis required.

#[cfg(test)]
mod redis_vector_conformance {
    use crate::backends::redis_vector::*;

    #[test]
    fn local_url_uses_redis_scheme() {
        let cfg = RedisVectorConfig::local();
        assert!(cfg.url().starts_with("redis://"), "url: {}", cfg.url());
    }

    #[test]
    fn tls_url_uses_rediss_scheme() {
        let cfg = RedisVectorConfig::new("myhost", 6380).with_tls();
        assert!(cfg.url().starts_with("rediss://"), "url: {}", cfg.url());
    }

    #[test]
    fn password_appears_in_url() {
        let cfg = RedisVectorConfig::new("myhost", 6379).with_password("s3cr3t");
        assert!(cfg.url().contains(":s3cr3t@"), "url: {}", cfg.url());
    }

    #[test]
    fn create_index_command_is_ft_create() {
        let idx = CreateIndexArgs::new("my_idx", "doc:", 256);
        assert_eq!(idx.to_json()["command"], "FT.CREATE");
    }

    #[test]
    fn create_index_flat_disables_hnsw_params() {
        let idx = CreateIndexArgs::new("idx", "doc:", 128).flat();
        assert!(idx.ef_construction.is_none());
        assert!(idx.m.is_none());
        assert_eq!(idx.algorithm, algorithm::FLAT);
    }

    #[test]
    fn create_index_hnsw_custom_params() {
        let idx = CreateIndexArgs::new("idx", "doc:", 128).hnsw_params(64, 8);
        assert_eq!(idx.ef_construction, Some(64));
        assert_eq!(idx.m, Some(8));
    }

    #[test]
    fn create_index_distance_override() {
        let idx = CreateIndexArgs::new("idx", "doc:", 128).distance(distance::L2);
        assert_eq!(idx.distance, distance::L2);
    }

    #[test]
    fn search_ann_query_filter_contains_knn() {
        let s = SearchArgs::ann("idx", "emb", 10);
        let j = s.to_json();
        assert!(
            j["query"].as_str().unwrap().contains("KNN"),
            "query: {}",
            j["query"]
        );
    }

    #[test]
    fn search_filtered_ann_has_pre_filter() {
        let s = SearchArgs::filtered_ann("idx", "@lang:{en}", "emb", 5);
        let q = s.to_json()["query"].as_str().unwrap().to_owned();
        assert!(q.contains("@lang:{en}"), "query: {q}");
    }

    #[test]
    fn search_returns_fields_propagate() {
        let s = SearchArgs::ann("idx", "emb", 5).returns(&["score", "title"]);
        let j = s.to_json();
        let ret = j["return"].as_array().unwrap();
        assert!(ret.iter().any(|v| v == "title"), "returns: {ret:?}");
    }

    #[test]
    fn tag_filter_brackets_value() {
        let f = tag_filter("lang", "en");
        assert_eq!(f, "@lang:{en}");
    }

    #[test]
    fn numeric_range_format_correct() {
        let f = numeric_range("score", 0.5, 1.0);
        assert_eq!(f, "@score:[0.5 1]");
    }

    #[test]
    fn document_key_joins_prefix_and_id() {
        assert_eq!(document_key("vec", 7), "vec:7");
    }

    #[test]
    fn error_index_already_exists_classified() {
        let e = RedisVectorError::from_redis_err("Index already exists");
        assert!(matches!(e, RedisVectorError::IndexAlreadyExists(_)));
    }

    #[test]
    fn error_oom_is_transient() {
        let e = RedisVectorError::from_redis_err("OOM command not allowed");
        assert!(e.is_transient());
    }

    #[test]
    fn error_unknown_index_is_index_not_found() {
        let e = RedisVectorError::from_redis_err("Unknown Index name");
        assert!(matches!(e, RedisVectorError::IndexNotFound(_)));
    }

    #[test]
    fn error_wrongtype_classified() {
        let e = RedisVectorError::from_redis_err("WRONGTYPE Operation against a key");
        assert!(matches!(e, RedisVectorError::WrongType(_)));
    }

    #[test]
    fn retry_delay_is_bounded() {
        assert!(redis_retry_delay_ms(0) <= 2_000);
        assert!(redis_retry_delay_ms(100) <= 2_000);
    }

    #[test]
    fn parse_search_results_extracts_key_and_score() {
        let body = serde_json::json!({
            "results": [
                { "key": "doc:1", "score": 0.88, "payload": "{}" }
            ]
        });
        let results = parse_search_results(&body);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "doc:1");
        assert!(
            (results[0].1 - 0.88f32).abs() < 1e-3,
            "score: {}",
            results[0].1
        );
    }

    #[test]
    fn parse_index_info_returns_name() {
        let body = serde_json::json!({ "index_name": "my_idx" });
        assert_eq!(parse_index_info(&body), Some("my_idx".to_owned()));
    }
}
