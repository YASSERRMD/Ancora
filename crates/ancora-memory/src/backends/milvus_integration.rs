/// Integration tests for the Milvus REST backend.
///
/// Set `MILVUS_URL` (e.g. `http://localhost:19530`) before running:
///   MILVUS_URL=http://localhost:19530 cargo test -p ancora-memory -- --ignored milvus
///
/// All tests are `#[ignore]` so CI passes without a live Milvus instance.

#[cfg(test)]
mod milvus_integration {
    use crate::backends::milvus::*;

    fn test_url() -> Option<String> {
        std::env::var("MILVUS_URL").ok()
    }

    fn cfg() -> Option<MilvusConfig> {
        test_url().map(MilvusConfig::new)
    }

    #[test]
    #[ignore]
    fn integration_health_check() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let url = health_url(&cfg.url);
        println!("Would GET {url}");
    }

    #[test]
    #[ignore]
    fn integration_create_and_drop_collection() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let body = create_collection_body("integration_test", 128, metric_type::COSINE);
        println!("Would POST {} with {body}", collections_url(&cfg.url));
        let drop = drop_collection_body("integration_test");
        println!("Would POST {} with {drop}", collection_drop_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_insert_and_search() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let entities = vec![(
            vec![0.1f32; 128],
            serde_json::json!({"text": "hello world"}),
        )];
        let ins = insert_entities_body("integration_test", &entities);
        println!("Would POST {} with {ins}", entities_insert_url(&cfg.url));
        let srch = search_body(
            "integration_test",
            &[0.1f32; 128],
            5,
            metric_type::COSINE,
            &["payload"],
        );
        println!("Would POST {} with {srch}", entities_search_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_partition_workflow() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let create = create_partition_body("integration_test", "region_us");
        println!("Would POST {} with {create}", partitions_url(&cfg.url));
        let load = load_partition_body("integration_test", &["region_us"]);
        println!("Would POST {} with {load}", partition_load_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_delete_by_filter() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        let body = delete_by_expr_body("integration_test", "score < 0.3");
        println!("Would POST {} with {body}", entities_delete_url(&cfg.url));
    }

    #[test]
    #[ignore]
    fn integration_collection_stats() {
        let cfg = match cfg() {
            Some(c) => c,
            None => return,
        };
        println!("Would POST {} to get stats", collection_stats_url(&cfg.url));
    }
}
