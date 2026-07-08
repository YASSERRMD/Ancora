/// Conformance tests for the LanceDB backend.
/// All offline -- no LanceDB process required.

#[cfg(test)]
mod lancedb_conformance {
    use crate::backends::lancedb::*;

    // ---- path conformance ------------------------------------------------

    #[test]
    fn local_path_reports_is_local() {
        let p = LanceDbPath::local("/tmp/test_db");
        assert!(p.is_local());
        assert!(!p.is_remote());
    }

    #[test]
    fn s3_path_reports_is_remote() {
        let p = LanceDbPath::s3("s3://bucket/key");
        assert!(p.is_remote());
        assert!(!p.is_local());
    }

    #[test]
    fn gcs_path_reports_is_remote() {
        let p = LanceDbPath::gcs("gs://b/k");
        assert!(p.is_remote());
    }

    #[test]
    fn azure_path_reports_is_remote() {
        let p = LanceDbPath::azure("az://c/b");
        assert!(p.is_remote());
    }

    // ---- schema conformance ----------------------------------------------

    #[test]
    fn table_schema_has_id_column() {
        let schema = table_schema(64, &[]);
        let cols = schema["columns"].as_array().unwrap();
        assert!(cols.iter().any(|c| c["name"] == "id"), "missing id");
    }

    #[test]
    fn table_schema_embedding_encodes_dims() {
        let schema = table_schema(256, &[]);
        let cols = schema["columns"].as_array().unwrap();
        let emb = cols.iter().find(|c| c["name"] == "embedding").unwrap();
        assert!(
            emb["type"].as_str().unwrap().contains("256"),
            "dims not in type"
        );
    }

    #[test]
    fn table_schema_extra_columns_appear_at_end() {
        let extra = ColumnDef::new("score", column_type::FLOAT32);
        let schema = table_schema(64, &[extra]);
        let cols = schema["columns"].as_array().unwrap();
        assert_eq!(cols.last().unwrap()["name"], "score");
    }

    // ---- row conformance -------------------------------------------------

    #[test]
    fn row_payload_is_serialized_to_string() {
        let r = row(1, vec![0.0f32], serde_json::json!({"tag": "x"}));
        assert!(r["payload"].is_string(), "payload must be a JSON string");
    }

    #[test]
    fn rows_batch_length_matches_input() {
        let data: Vec<_> = (0..5)
            .map(|i| (i as i64, vec![0.1f32], serde_json::json!({})))
            .collect();
        assert_eq!(rows(&data).len(), 5);
    }

    // ---- query conformance -----------------------------------------------

    #[test]
    fn vector_query_default_metric_is_cosine() {
        let q = VectorQuery::new("t", vec![0.0f32], 5);
        assert_eq!(q.metric, "cosine");
    }

    #[test]
    fn vector_query_json_includes_select() {
        let q = VectorQuery::new("t", vec![0.0f32], 5).select(&["id", "score"]);
        let j = q.to_json();
        let sel = j["select"].as_array().unwrap();
        assert_eq!(sel.len(), 2);
    }

    #[test]
    fn vector_query_refine_factor_is_included() {
        let q = VectorQuery::new("t", vec![0.0f32], 5).refine(4);
        assert_eq!(q.to_json()["refine_factor"], 4);
    }

    // ---- delete conformance ----------------------------------------------

    #[test]
    fn delete_predicate_sets_both_fields() {
        let j = delete_predicate("t", "id > 100");
        assert_eq!(j["table"], "t");
        assert_eq!(j["predicate"], "id > 100");
    }

    // ---- storage type conformance ----------------------------------------

    #[test]
    fn object_storage_path_detected_correctly() {
        assert_eq!(detect_storage_type("s3://b/k"), "s3");
        assert_eq!(detect_storage_type("gs://b/k"), "gcs");
        assert_eq!(detect_storage_type("az://c/b"), "azure");
        assert_eq!(detect_storage_type("/local/path"), "local");
    }

    // ---- edge default conformance ----------------------------------------

    #[test]
    fn edge_default_dir_returns_a_string() {
        let d = edge_default_dir();
        assert!(!d.is_empty());
    }
}
