/// Conformance tests for the Pinecone backend with mock responses.
/// All offline.

#[cfg(test)]
mod pinecone_conformance {
    use crate::backends::pinecone::*;

    #[test]
    fn serverless_index_body_has_spec() {
        let body = create_serverless_index_body("idx", 768, metric::COSINE, "aws", "us-east-1");
        assert!(body["spec"]["serverless"].is_object());
        assert_eq!(body["dimension"], 768);
    }

    #[test]
    fn pod_index_body_has_replicas() {
        let body = create_pod_index_body("idx", 384, metric::DOT_PRODUCT, "s1.x1", 3);
        assert_eq!(body["spec"]["pod"]["replicas"], 3);
    }

    #[test]
    fn upsert_body_ids_in_vectors() {
        let body = upsert_body(&[("v1", vec![0.1f32], serde_json::json!({}))]);
        assert_eq!(body["vectors"][0]["id"], "v1");
    }

    #[test]
    fn upsert_namespace_body_has_namespace() {
        let body = upsert_namespace_body(&[("v1", vec![0.1f32], serde_json::json!({}))], "ns1");
        assert_eq!(body["namespace"], "ns1");
    }

    #[test]
    fn query_body_top_k_propagates() {
        let body = query_body(&[0.1f32], 20, None, false);
        assert_eq!(body["topK"], 20);
    }

    #[test]
    fn query_body_include_metadata_propagates() {
        let body = query_body(&[0.1f32], 5, None, true);
        assert_eq!(body["includeMetadata"], true);
    }

    #[test]
    fn delete_ids_body_wraps_ids() {
        let body = delete_ids_body(&["id1", "id2"]);
        assert_eq!(body["ids"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn filter_or_produces_or_key() {
        let f = filter_or(vec![filter_eq("a", serde_json::json!(1))]);
        assert!(f["$or"].is_array());
    }

    #[test]
    fn parse_matches_extracts_id_and_score() {
        let body = serde_json::json!({
            "matches": [{ "id": "v1", "score": 0.95, "metadata": {"tag": "x"} }]
        });
        let results = parse_matches(&body);
        assert_eq!(results[0].0, "v1");
        assert!((results[0].1 - 0.95f32).abs() < 1e-4);
    }

    #[test]
    fn parse_index_host_extracts_string() {
        let body = serde_json::json!({ "host": "idx-abc.svc.pinecone.io" });
        assert_eq!(
            parse_index_host(&body),
            Some("idx-abc.svc.pinecone.io".to_owned())
        );
    }
}
