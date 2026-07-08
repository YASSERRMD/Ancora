/// Conformance tests for the Milvus backend request-building layer.
/// All offline -- no live Milvus required.

#[cfg(test)]
mod milvus_conformance {
    use crate::backends::milvus::*;

    // ---- schema conformance ----------------------------------------------

    #[test]
    fn create_collection_has_primary_key() {
        let body = create_collection_body("test", 128, metric_type::COSINE);
        let fields = body["schema"]["fields"].as_array().unwrap();
        let pk = fields.iter().find(|f| f["isPrimary"] == true);
        assert!(pk.is_some(), "schema must have a primary key field");
    }

    #[test]
    fn create_collection_has_float_vector_field() {
        let body = create_collection_body("test", 128, metric_type::COSINE);
        let fields = body["schema"]["fields"].as_array().unwrap();
        let vec_field = fields
            .iter()
            .find(|f| f["dataType"] == field_type::FLOAT_VECTOR);
        assert!(
            vec_field.is_some(),
            "schema must include a FloatVector field"
        );
    }

    #[test]
    fn create_collection_embeds_index_params() {
        let body = create_collection_body("test", 256, metric_type::L2);
        let idx = body["indexParams"].as_array().unwrap();
        assert!(!idx.is_empty(), "indexParams must not be empty");
    }

    // ---- insert conformance ----------------------------------------------

    #[test]
    fn insert_body_includes_collection_name() {
        let body = insert_entities_body("my_col", &[(vec![0.1f32; 4], serde_json::json!({}))]);
        assert_eq!(body["collectionName"], "my_col");
    }

    #[test]
    fn insert_body_has_data_array() {
        let body = insert_entities_body(
            "c",
            &[
                (vec![0.1f32; 4], serde_json::json!({"k": "v"})),
                (vec![0.2f32; 4], serde_json::json!({"k": "w"})),
            ],
        );
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
    }

    // ---- search conformance ----------------------------------------------

    #[test]
    fn search_body_includes_limit() {
        let body = search_body("c", &[0.0f32; 4], 10, metric_type::COSINE, &["payload"]);
        assert_eq!(body["limit"], 10);
    }

    #[test]
    fn search_body_includes_metric_type() {
        let body = search_body("c", &[0.0f32; 4], 10, metric_type::IP, &["payload"]);
        assert_eq!(body["searchParams"]["metric_type"], metric_type::IP);
    }

    #[test]
    fn search_body_has_data_array() {
        let body = search_body("c", &[0.1f32; 4], 5, metric_type::L2, &["payload"]);
        let data = body["data"].as_array().unwrap();
        assert_eq!(data.len(), 1, "one query vector");
    }

    // ---- delete conformance ----------------------------------------------

    #[test]
    fn delete_by_expr_sets_filter() {
        let body = delete_by_expr_body("c", "score < 0.5");
        assert_eq!(body["filter"], "score < 0.5");
        assert_eq!(body["collectionName"], "c");
    }

    #[test]
    fn delete_by_ids_produces_in_expr() {
        let body = delete_by_ids_body("c", &[1, 2]);
        let f = body["filter"].as_str().unwrap();
        assert!(f.starts_with("id in ["), "filter: {f}");
    }

    // ---- partition conformance -------------------------------------------

    #[test]
    fn create_partition_body_has_both_fields() {
        let body = create_partition_body("c", "shard_0");
        assert_eq!(body["collectionName"], "c");
        assert_eq!(body["partitionName"], "shard_0");
    }

    #[test]
    fn load_partition_body_accepts_multiple() {
        let body = load_partition_body("c", &["p0", "p1"]);
        let parts = body["partitionNames"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
    }

    // ---- index conformance -----------------------------------------------

    #[test]
    fn create_index_body_sets_index_type() {
        let body = create_index_body(
            "c",
            "embedding",
            "my_idx",
            index_type::HNSW,
            metric_type::COSINE,
        );
        let idx = &body["indexParams"][0];
        assert_eq!(idx["indexType"], index_type::HNSW);
    }

    // ---- response parsing conformance ------------------------------------

    #[test]
    fn parse_search_results_empty_on_missing_data() {
        let body = serde_json::json!({});
        let results = parse_search_results(&body);
        assert!(results.is_empty());
    }

    #[test]
    fn parse_query_results_returns_id_and_payload() {
        let body = serde_json::json!({
            "data": [{ "id": 7, "payload": r#"{"tag":"x"}"# }]
        });
        let rows = parse_query_results(&body);
        assert_eq!(rows[0].0, 7);
    }

    // ---- consistency level conformance -----------------------------------

    #[test]
    fn consistency_constants_match_milvus_names() {
        assert_eq!(consistency::STRONG, "Strong");
        assert_eq!(consistency::BOUNDED, "Bounded");
        assert_eq!(consistency::SESSION, "Session");
        assert_eq!(consistency::EVENTUALLY, "Eventually");
    }

    // ---- metric and field type constants ---------------------------------

    #[test]
    fn metric_constants_are_correct() {
        assert_eq!(metric_type::COSINE, "COSINE");
        assert_eq!(metric_type::IP, "IP");
        assert_eq!(metric_type::L2, "L2");
    }

    // ---- extended response parser conformance ---------------------------

    #[test]
    fn parse_error_message_extracts_message() {
        let body = serde_json::json!({ "message": "collection not loaded" });
        assert_eq!(
            parse_error_message(&body),
            Some("collection not loaded".to_owned())
        );
    }

    #[test]
    fn parse_error_message_returns_none_on_missing() {
        let body = serde_json::json!({ "data": {} });
        assert!(parse_error_message(&body).is_none());
    }

    #[test]
    fn parse_delete_count_reads_field() {
        let body = serde_json::json!({ "data": { "deleteCount": 7 } });
        assert_eq!(parse_delete_count(&body), 7);
    }

    #[test]
    fn parse_delete_count_zero_on_missing() {
        let body = serde_json::json!({});
        assert_eq!(parse_delete_count(&body), 0);
    }

    #[test]
    fn parse_alias_names_extracts_list() {
        let body = serde_json::json!({
            "data": [{ "aliasName": "latest" }, { "aliasName": "staging" }]
        });
        let names = parse_alias_names(&body);
        assert_eq!(names, vec!["latest", "staging"]);
    }

    #[test]
    fn upsert_entities_body_sets_explicit_ids() {
        let entities = vec![(42i64, vec![0.1f32; 4], serde_json::json!({"tag": "x"}))];
        let body = upsert_entities_body("col", &entities);
        let id = body["data"][0]["id"].as_i64().unwrap();
        assert_eq!(id, 42);
    }
}
