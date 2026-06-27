/// Conformance surface tests for the Qdrant REST layer.
///
/// These tests verify that every HTTP request shape produced by the qdrant module
/// is structurally correct: correct URL form, correct JSON shape, correct field
/// names and types. No live Qdrant server is required.

#[cfg(test)]
mod qdrant_conformance {
    use crate::backends::qdrant::*;
    use crate::vector_store::{Distance, Filter, PayloadValue};

    // -- Collection lifecycle surface ----------------------------------------

    #[test]
    fn conformance_create_single_vector_body() {
        let body = create_collection_body(1536, &Distance::Cosine);
        assert!(body["vectors"]["size"].is_number());
        assert!(body["vectors"]["distance"].is_string());
    }

    #[test]
    fn conformance_create_multi_vector_all_distances() {
        let body = create_multi_vector_collection_body(&[
            ("text", 384, Distance::Cosine),
            ("img", 512, Distance::L2),
            ("emb", 768, Distance::Dot),
        ]);
        assert_eq!(body["vectors"]["text"]["distance"], "Cosine");
        assert_eq!(body["vectors"]["img"]["distance"], "Euclid");
        assert_eq!(body["vectors"]["emb"]["distance"], "Dot");
    }

    // -- URL surface ---------------------------------------------------------

    #[test]
    fn conformance_all_collection_urls_include_collection_name() {
        let base = "http://localhost:6333";
        let name = "test_col";
        for url in [
            collection_url(base, name),
            points_url(base, name),
            upsert_url(base, name),
            search_url(base, name),
            delete_points_url(base, name),
            scroll_url(base, name),
        ] {
            assert!(url.contains(name), "url missing name: {url}");
        }
    }

    // -- Upsert surface ------------------------------------------------------

    #[test]
    fn conformance_upsert_body_every_point_has_id_vector_payload() {
        let pts = vec![
            (1u64, vec![0.1f32, 0.2], serde_json::json!({})),
            (2u64, vec![0.3f32, 0.4], serde_json::json!({ "k": "v" })),
        ];
        let body = upsert_body(&pts);
        for pt in body["points"].as_array().unwrap() {
            assert!(pt["id"].is_number());
            assert!(pt["vector"].is_array());
            assert!(pt["payload"].is_object());
        }
    }

    // -- Search surface ------------------------------------------------------

    #[test]
    fn conformance_search_body_has_limit_and_with_payload() {
        let body = search_body(&[0.1f32, 0.2], 10, None);
        assert_eq!(body["limit"], 10);
        assert_eq!(body["with_payload"], true);
    }

    #[test]
    fn conformance_search_with_filter_has_filter_key() {
        let f = Filter::Eq("lang".to_owned(), PayloadValue::String("en".to_owned()));
        let body = search_body(&[0.1f32], 5, Some(&f));
        assert!(body["filter"].is_object());
    }

    // -- Delete surface ------------------------------------------------------

    #[test]
    fn conformance_delete_by_filter_body_has_filter() {
        let f = Filter::Gt("age".to_owned(), PayloadValue::Integer(30));
        let body = delete_by_filter_body(&f);
        assert!(body["filter"]["must"].is_array());
    }

    // -- Filter surface ------------------------------------------------------

    #[test]
    fn conformance_filter_lt_produces_lt_range() {
        let f = Filter::Lt("year".to_owned(), PayloadValue::Integer(2000));
        let json = filter_to_qdrant(&f);
        assert!(json["must"][0]["range"]["lt"].is_number());
    }

    #[test]
    fn conformance_filter_nested_and_or() {
        let f = Filter::Eq("a".to_owned(), PayloadValue::Integer(1))
            .and(Filter::Eq("b".to_owned(), PayloadValue::Integer(2)))
            .or(Filter::Eq("c".to_owned(), PayloadValue::Integer(3)));
        let json = filter_to_qdrant(&f);
        assert!(json["should"].is_array(), "json: {json}");
    }

    // -- Error surface -------------------------------------------------------

    #[test]
    fn conformance_unauthorized_error() {
        let err = QdrantError::from_response(401, "{}");
        assert_eq!(err, QdrantError::Unauthorized);
        assert!(!err.is_transient());
    }

    // -- Scroll surface ------------------------------------------------------

    #[test]
    fn conformance_scroll_body_without_offset_has_no_offset_key() {
        let body = scroll_body(10, None);
        assert!(body["offset"].is_null() || !body.as_object().unwrap().contains_key("offset"));
    }
}
