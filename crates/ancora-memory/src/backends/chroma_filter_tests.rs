/// Extended metadata filter tests for the Chroma backend.
/// All offline.

#[cfg(test)]
mod chroma_filter_tests {
    use crate::backends::chroma::*;

    // ---- where_eq / where_ne -----------------------------------------------

    #[test]
    fn where_eq_string_value() {
        let f = where_eq("category", serde_json::json!("news"));
        assert_eq!(f["category"]["$eq"], "news");
    }

    #[test]
    fn where_eq_numeric_value() {
        let f = where_eq("score", serde_json::json!(42));
        assert_eq!(f["score"]["$eq"], 42);
    }

    #[test]
    fn where_ne_numeric_value() {
        let f = where_ne("priority", serde_json::json!(0));
        assert_eq!(f["priority"]["$ne"], 0);
    }

    // ---- where_gt / where_lt -----------------------------------------------

    #[test]
    fn where_gt_produces_gt_key() {
        let f = where_gt("score", serde_json::json!(0.7));
        assert!(f["score"]["$gt"].as_f64().unwrap() > 0.5);
    }

    #[test]
    fn where_lt_produces_lt_key() {
        let f = where_lt("rank", serde_json::json!(100));
        assert_eq!(f["rank"]["$lt"], 100);
    }

    // ---- where_and / where_or ----------------------------------------------

    #[test]
    fn where_and_wraps_two_conditions() {
        let f = where_and(
            where_eq("lang", serde_json::json!("en")),
            where_gt("score", serde_json::json!(0.5)),
        );
        let arr = f["$and"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn where_or_wraps_two_conditions() {
        let f = where_or(
            where_eq("tag", serde_json::json!("rust")),
            where_eq("tag", serde_json::json!("python")),
        );
        let arr = f["$or"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn nested_and_or_composes() {
        // (lang == "en" OR lang == "fr") AND score > 0.6
        let lang_filter = where_or(
            where_eq("lang", serde_json::json!("en")),
            where_eq("lang", serde_json::json!("fr")),
        );
        let score_filter = where_gt("score", serde_json::json!(0.6));
        let combined = where_and(lang_filter, score_filter);
        assert!(combined["$and"].is_array());
    }

    // ---- where_in ----------------------------------------------------------

    #[test]
    fn where_in_wraps_array() {
        let f = where_in("category", vec![
            serde_json::json!("news"),
            serde_json::json!("sports"),
        ]);
        let arr = f["category"]["$in"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert!(arr.contains(&serde_json::json!("sports")));
    }

    #[test]
    fn where_in_single_element() {
        let f = where_in("status", vec![serde_json::json!("active")]);
        assert_eq!(f["status"]["$in"].as_array().unwrap().len(), 1);
    }

    // ---- query_body with where filter --------------------------------------

    #[test]
    fn query_body_passes_where_filter() {
        let filter = where_eq("lang", serde_json::json!("en"));
        let body = query_body(&[vec![0.1f32]], 5, Some(filter), &["distances"]);
        assert!(body["where"].is_object(), "where must be present when filter given");
    }

    #[test]
    fn query_body_no_where_when_none() {
        let body = query_body(&[vec![0.1f32]], 5, None, &["distances"]);
        assert!(body["where"].is_null(), "where should be absent when filter is None");
    }

    #[test]
    fn query_body_multiple_embeddings() {
        let embs = vec![vec![0.1f32, 0.2f32], vec![0.3f32, 0.4f32]];
        let body = query_body(&embs, 3, None, &["ids"]);
        assert_eq!(body["query_embeddings"].as_array().unwrap().len(), 2);
    }

    // ---- delete_body with filter -------------------------------------------

    #[test]
    fn delete_body_ids_only() {
        let body = delete_body(&["id1", "id2"], None);
        assert_eq!(body["ids"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn delete_body_filter_only() {
        let f = where_eq("obsolete", serde_json::json!(true));
        let body = delete_body(&[], Some(f));
        assert!(body["where"].is_object());
    }

    // ---- add_body with documents -------------------------------------------

    #[test]
    fn add_body_with_documents() {
        let body = add_body(
            &["a"],
            &[vec![0.1f32]],
            &[serde_json::json!({"k": "v"})],
            Some(&["doc text"]),
        );
        let docs = body["documents"].as_array().unwrap();
        assert_eq!(docs[0], "doc text");
    }

    #[test]
    fn add_body_without_documents_omits_key() {
        let body = add_body(
            &["a"],
            &[vec![0.1f32]],
            &[serde_json::json!({})],
            None,
        );
        assert!(body["documents"].is_null());
    }

    // ---- create_collection_body --------------------------------------------

    #[test]
    fn create_collection_body_with_metadata() {
        let meta = serde_json::json!({"hnsw:space": "cosine"});
        let body = create_collection_body("col", Some(meta));
        assert_eq!(body["metadata"]["hnsw:space"], "cosine");
    }

    #[test]
    fn create_collection_body_without_metadata() {
        let body = create_collection_body("col", None);
        assert!(body["metadata"].is_null());
    }
}
