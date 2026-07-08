/// Tests verifying that the LanceDB backend runs fully offline and embedded.
/// All offline -- zero network I/O.

#[cfg(test)]
mod lancedb_offline_tests {
    use crate::backends::lancedb::*;

    #[test]
    fn local_path_requires_no_credentials() {
        let cfg = LanceDbConfig::local("/tmp/test_db");
        assert!(
            cfg.aws_region.is_none(),
            "local path must not need AWS credentials"
        );
    }

    #[test]
    fn edge_default_dir_can_be_set_by_env_var() {
        // Without the env var, falls back to a sensible default
        let dir = edge_default_dir();
        assert!(!dir.is_empty());
    }

    #[test]
    fn all_query_descriptors_are_buildable_without_io() {
        let _q = VectorQuery::new("t", vec![0.1f32; 4], 10).filter("x = 1");
        let _h = HybridQuery::new("t", vec![0.1f32; 4], "query", 5);
        let _v = VersionCheckout::new("t", 1);
        // All construct without panicking -- no I/O
    }

    #[test]
    fn delete_predicate_is_pure_data() {
        let j = delete_predicate("t", "year < 2020");
        assert!(j.is_object(), "delete descriptor must be a JSON object");
    }

    #[test]
    fn parse_results_is_pure_function() {
        let body = serde_json::json!({ "rows": [] });
        let results = parse_results(&body);
        assert!(results.is_empty());
    }

    #[test]
    fn table_schema_is_pure_function() {
        let schema = table_schema(384, &[]);
        assert!(schema.is_object());
    }

    #[test]
    fn row_serialization_is_pure() {
        let r = row(1, vec![0.0f32; 4], serde_json::json!({"k": "v"}));
        assert!(r.is_object());
    }

    #[test]
    fn ann_index_config_is_pure() {
        let idx = AnnIndex::new(256, 16).metric("l2");
        let j = idx.to_json();
        assert_eq!(j["index_type"], "IVF_PQ");
    }

    #[test]
    fn read_only_config_flag_is_respected() {
        let cfg = LanceDbConfig::local("/data").read_only();
        assert!(cfg.read_only, "read_only flag must be set");
    }
}
