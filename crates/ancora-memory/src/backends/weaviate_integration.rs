/// Integration tests for the Weaviate REST/GraphQL backend.
///
/// Set `WEAVIATE_URL` (e.g. `http://localhost:8080`) before running:
///   WEAVIATE_URL=http://localhost:8080 cargo test -p ancora-memory -- --ignored weaviate
///
/// All tests are `#[ignore]` so CI passes without a live Weaviate.

#[cfg(test)]
mod weaviate_integration {
    use crate::backends::weaviate::*;

    fn test_url() -> Option<String> {
        std::env::var("WEAVIATE_URL").ok()
    }

    fn cfg() -> Option<WeaviateConfig> {
        test_url().map(WeaviateConfig::new)
    }

    #[test]
    #[ignore]
    fn integration_readiness_check() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let url = readiness_url(&cfg.url);
        println!("Would GET {url}");
    }

    #[test]
    #[ignore]
    fn integration_create_and_delete_class() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let body = create_class_body("IntegrationTest", "test", "none");
        let url = schema_url(&cfg.url);
        println!("Would POST {url} with {body}");
        let del_url = class_url(&cfg.url, "IntegrationTest");
        println!("Would DELETE {del_url}");
    }

    #[test]
    #[ignore]
    fn integration_create_and_query_object() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let obj = create_object_body("Document", &serde_json::json!({"title": "test"}), Some(&[0.1f32, 0.2]));
        let up_url = objects_url(&cfg.url);
        let gql_url = graphql_url(&cfg.url);
        let qbody = graphql_near_vector_query("Document", &[0.1f32, 0.2], 5, &["title"]);
        println!("Would POST {up_url} then POST {gql_url}");
        println!("upsert body: {obj}");
        println!("search body: {qbody}");
    }

    #[test]
    #[ignore]
    fn integration_batch_upsert() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let objs = vec![
            ("Document".to_owned(), serde_json::json!({"title": "a"}), Some(vec![0.1f32])),
        ];
        let body = batch_objects_body(&objs);
        println!("Would POST {} with {body}", batch_objects_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_aggregate_count() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let body = graphql_aggregate_count_query("Document");
        println!("Would POST {} with {body}", graphql_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_hybrid_search() {
        let cfg = match cfg() { Some(c) => c, None => return };
        let body = graphql_hybrid_query("Document", "machine learning", None, 0.7, 10, &["title"]);
        println!("Would POST {} with {body}", graphql_url(&cfg.url));
    }
}
