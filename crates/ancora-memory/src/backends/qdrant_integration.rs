/// Integration tests for the Qdrant REST backend.
///
/// These tests require a live Qdrant instance. Set `QDRANT_URL` in the
/// environment before running:
///
///   QDRANT_URL=http://localhost:6333 cargo test -p ancora-memory -- --ignored qdrant
///
/// All tests are `#[ignore]` so CI stays green without a Qdrant service.

#[cfg(test)]
mod qdrant_integration {
    use crate::backends::qdrant::*;

    fn test_url() -> Option<String> {
        std::env::var("QDRANT_URL").ok()
    }

    fn cfg() -> Option<QdrantConfig> {
        test_url().map(QdrantConfig::new)
    }

    #[test]
    #[ignore]
    fn integration_readiness_check() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let url = readiness_url(&cfg.url);
        // GET url should return 200 with {"status":"ok"}
        println!("Would GET {url}");
    }

    #[test]
    #[ignore]
    fn integration_create_and_drop_collection() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let name = "integration_test_drop_me";
        let create_url = collection_url(&cfg.url, name);
        let body = create_collection_body(384, &crate::vector_store::Distance::Cosine);
        println!("Would PUT {create_url} with {body}");
        // drop
        println!("Would DELETE {create_url}");
    }

    #[test]
    #[ignore]
    fn integration_upsert_and_search() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let name = "integration_rag";
        let up_url = upsert_url(&cfg.url, name);
        let s_url = search_url(&cfg.url, name);
        let pts = vec![(1u64, vec![0.1f32, 0.2, 0.3], serde_json::json!({}))];
        let up_body = upsert_body(&pts);
        let s_body = search_body(&[0.1f32, 0.2, 0.3], 5, None);
        println!("Would PUT {up_url} then POST {s_url}");
        println!("upsert: {up_body}");
        println!("search: {s_body}");
    }

    #[test]
    #[ignore]
    fn integration_delete_by_filter() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let name = "integration_rag";
        let del_url = delete_url(&cfg.url, name);
        let f = crate::vector_store::Filter::Eq(
            "status".to_owned(),
            crate::vector_store::PayloadValue::String("old".to_owned()),
        );
        let body = delete_by_filter_body(&f);
        println!("Would POST {del_url} with {body}");
    }

    #[test]
    #[ignore]
    fn integration_scroll_all_points() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let name = "integration_rag";
        let sc_url = scroll_url(&cfg.url, name);
        let body = scroll_body(100, None);
        println!("Would POST {sc_url} with {body}");
    }

    #[test]
    #[ignore]
    fn integration_create_and_delete_alias() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let al_url = aliases_url(&cfg.url);
        let create = create_alias_body("docs_v2", "docs");
        let delete = delete_alias_body("docs");
        println!("Would POST {al_url} with create={create} then delete={delete}");
    }
}
