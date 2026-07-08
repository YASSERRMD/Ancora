/// Vector search configuration tests for the LanceDB backend.
/// All offline.

#[cfg(test)]
mod lancedb_vector_search_tests {
    use crate::backends::lancedb::*;

    #[test]
    fn vector_query_refine_factor_is_set() {
        let q = VectorQuery::new("t", vec![0.1f32], 5).refine(8);
        assert_eq!(q.to_json()["refine_factor"], 8);
    }

    #[test]
    fn vector_query_refine_factor_absent_when_not_set() {
        let q = VectorQuery::new("t", vec![0.1f32], 5);
        assert!(q.to_json()["refine_factor"].is_null());
    }

    #[test]
    fn vector_query_ef_absent_when_not_set() {
        let q = VectorQuery::new("t", vec![0.1f32], 5);
        assert!(q.to_json()["ef"].is_null());
    }

    #[test]
    fn vector_query_select_columns_default() {
        let q = VectorQuery::new("t", vec![0.1f32], 5);
        let body = q.to_json();
        let sel = body["select"].as_array().unwrap();
        assert!(
            sel.iter().any(|s| s == "id"),
            "default select must include id"
        );
    }

    #[test]
    fn vector_query_select_override() {
        let q = VectorQuery::new("t", vec![0.1f32], 5).select(&["title", "year"]);
        let body = q.to_json();
        let sel = body["select"].as_array().unwrap();
        assert_eq!(sel.len(), 2);
    }

    #[test]
    fn vector_query_metric_override_propagates() {
        let q = VectorQuery::new("t", vec![0.1f32], 5).metric("l2");
        assert_eq!(q.to_json()["metric"], "l2");
    }

    #[test]
    fn parse_results_distance_extracted() {
        let body =
            serde_json::json!({ "rows": [{ "id": 1, "_distance": 0.25, "payload": r#"{}"# }] });
        let results = parse_results(&body);
        assert!(
            (results[0].1 - 0.25f32).abs() < 1e-5,
            "score: {}",
            results[0].1
        );
    }

    #[test]
    fn parse_results_empty_body_gives_empty_vec() {
        let body = serde_json::json!({});
        assert!(parse_results(&body).is_empty());
    }
}
