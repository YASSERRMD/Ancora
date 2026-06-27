/// Pinecone index operations tests -- all offline.
/// Covers index creation bodies, upsert, query, delete, and filter helpers.

#[cfg(test)]
mod pinecone_index_tests {
    use crate::backends::pinecone::*;

    // ---- serverless index body -----------------------------------------

    #[test]
    fn serverless_body_cloud_and_region() {
        let b = create_serverless_index_body("idx", 768, metric::COSINE, "aws", "us-east-1");
        assert_eq!(b["spec"]["serverless"]["cloud"], "aws");
        assert_eq!(b["spec"]["serverless"]["region"], "us-east-1");
    }

    #[test]
    fn serverless_body_name_and_dimension() {
        let b = create_serverless_index_body("my_idx", 384, metric::COSINE, "gcp", "us-central1");
        assert_eq!(b["name"], "my_idx");
        assert_eq!(b["dimension"], 384);
    }

    #[test]
    fn serverless_body_metric_dot_product() {
        let b = create_serverless_index_body("idx", 128, metric::DOT_PRODUCT, "aws", "eu-west-1");
        assert_eq!(b["metric"], metric::DOT_PRODUCT);
    }

    // ---- pod index body ------------------------------------------------

    #[test]
    fn pod_body_pod_type_stored() {
        let b = create_pod_index_body("idx", 256, metric::EUCLIDEAN, "p1.x2", 1);
        assert_eq!(b["spec"]["pod"]["pod_type"], "p1.x2");
    }

    #[test]
    fn pod_body_replicas_stored() {
        let b = create_pod_index_body("idx", 256, metric::EUCLIDEAN, "p1.x1", 4);
        assert_eq!(b["spec"]["pod"]["replicas"], 4);
    }

    // ---- upsert body ---------------------------------------------------

    #[test]
    fn upsert_body_vector_count() {
        let vecs = vec![
            ("v1", vec![0.1f32, 0.2f32], serde_json::json!({"a": 1})),
            ("v2", vec![0.3f32, 0.4f32], serde_json::json!({"a": 2})),
        ];
        let b = upsert_body(&vecs);
        assert_eq!(b["vectors"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn upsert_body_id_and_values() {
        let b = upsert_body(&[("vec1", vec![0.5f32], serde_json::json!({}))]);
        assert_eq!(b["vectors"][0]["id"], "vec1");
        assert!(b["vectors"][0]["values"].as_array().is_some());
    }

    #[test]
    fn upsert_body_metadata_stored() {
        let b = upsert_body(&[("v1", vec![0.1f32], serde_json::json!({"lang": "en"}))]);
        assert_eq!(b["vectors"][0]["metadata"]["lang"], "en");
    }

    #[test]
    fn upsert_namespace_body_sets_namespace() {
        let b = upsert_namespace_body(&[("v1", vec![0.1f32], serde_json::json!({}))], "my-ns");
        assert_eq!(b["namespace"], "my-ns");
    }

    // ---- query body ----------------------------------------------------

    #[test]
    fn query_body_top_k() {
        let b = query_body(&[0.1f32, 0.2f32], 25, None, true);
        assert_eq!(b["topK"], 25);
    }

    #[test]
    fn query_body_include_metadata_propagates() {
        let b = query_body(&[0.1f32], 5, None, true);
        assert_eq!(b["includeMetadata"], true);
        let b2 = query_body(&[0.1f32], 5, None, false);
        assert_eq!(b2["includeMetadata"], false);
    }

    #[test]
    fn query_body_with_filter() {
        let f = filter_eq("lang", serde_json::json!("en"));
        let b = query_body(&[0.1f32], 10, Some(f), true);
        assert!(b["filter"].is_object());
    }

    #[test]
    fn query_namespace_body_has_namespace() {
        let b = query_namespace_body(&[0.1f32], 5, "zone-a");
        assert_eq!(b["namespace"], "zone-a");
    }

    // ---- delete bodies -------------------------------------------------

    #[test]
    fn delete_ids_body_list() {
        let b = delete_ids_body(&["x", "y", "z"]);
        assert_eq!(b["ids"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn delete_filter_body_has_filter() {
        let f = filter_eq("obsolete", serde_json::json!(true));
        let b = delete_filter_body(f);
        assert!(b["filter"].is_object());
    }

    #[test]
    fn delete_namespace_body_has_delete_all() {
        let b = delete_namespace_body("old-ns");
        assert_eq!(b["deleteAll"], true);
        assert_eq!(b["namespace"], "old-ns");
    }

    // ---- filter helpers ------------------------------------------------

    #[test]
    fn filter_ne_produces_ne_key() {
        let f = filter_ne("status", serde_json::json!("deleted"));
        assert_eq!(f["status"]["$ne"], "deleted");
    }

    #[test]
    fn filter_gte_produces_gte_key() {
        let f = filter_gte("year", serde_json::json!(2020));
        assert_eq!(f["year"]["$gte"], 2020);
    }

    #[test]
    fn filter_lte_produces_lte_key() {
        let f = filter_lte("rank", serde_json::json!(100));
        assert_eq!(f["rank"]["$lte"], 100);
    }

    #[test]
    fn filter_and_has_and_key() {
        let f = filter_and(vec![
            filter_eq("a", serde_json::json!(1)),
            filter_eq("b", serde_json::json!(2)),
        ]);
        assert_eq!(f["$and"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_in_has_in_key() {
        let f = filter_in("cat", vec![serde_json::json!("x"), serde_json::json!("y")]);
        assert_eq!(f["cat"]["$in"].as_array().unwrap().len(), 2);
    }

    // ---- response parsing ----------------------------------------------

    #[test]
    fn parse_matches_score_and_metadata() {
        let body = serde_json::json!({
            "matches": [
                { "id": "v1", "score": 0.92, "metadata": {"tag": "rust"} },
                { "id": "v2", "score": 0.85, "metadata": {} }
            ]
        });
        let m = parse_matches(&body);
        assert_eq!(m.len(), 2);
        assert_eq!(m[0].0, "v1");
        assert!((m[0].1 - 0.92f32).abs() < 1e-4);
    }

    #[test]
    fn parse_index_host_returns_host() {
        let b = serde_json::json!({ "host": "idx-abc.svc.pinecone.io" });
        assert_eq!(parse_index_host(&b), Some("idx-abc.svc.pinecone.io".to_owned()));
    }

    #[test]
    fn parse_index_stats_returns_total_and_dims() {
        let b = serde_json::json!({ "totalVectorCount": 50000, "dimension": 768 });
        let (total, dims) = parse_index_stats(&b);
        assert_eq!(total, 50000);
        assert_eq!(dims, 768);
    }

    // ---- retry helpers -------------------------------------------------

    #[test]
    fn pinecone_retry_delay_is_bounded() {
        assert!(pinecone_retry_delay_ms(0) >= 1);
        assert!(pinecone_retry_delay_ms(100) <= 10_000);
    }
}
