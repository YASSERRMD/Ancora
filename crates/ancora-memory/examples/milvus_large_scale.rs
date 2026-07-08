/// Milvus large-scale configuration example.
///
/// Demonstrates how to configure collections for billion-scale workloads:
/// IVF_FLAT vs HNSW index selection, partition strategies, and consistency
/// level trade-offs.
///
/// Set MILVUS_URL to run against a real Milvus instance.
/// Without the env var the example prints request shapes and exits cleanly.
use ancora_memory::backends::milvus::{
    collection_load_url, collections_url, consistency, create_collection_hnsw_body,
    create_collection_ivf_body, create_partition_body, delete_by_expr_body, entities_delete_url,
    entities_hybrid_url, entities_insert_url, entities_search_url, hybrid_search_body, index_type,
    insert_into_partition_body, load_collection_body, load_partition_body, metric_type,
    partition_load_url, partitions_url, recommended_nlist, search_hnsw_body, search_partition_body,
    search_with_consistency_body, sizing_guidance, MilvusConfig,
};

const COLLECTION: &str = "ArticleIndex";
const DIMS: usize = 768;
const EXPECTED_ROWS: u64 = 100_000_000; // 100M

fn main() {
    println!("=== Milvus large-scale config example ===\n");

    let cfg = MilvusConfig::local();
    println!("URL: {}\n", cfg.url);

    // 1. Sizing guidance
    let guidance = sizing_guidance(DIMS, EXPECTED_ROWS);
    println!("-- Sizing guidance --\n{guidance}\n");
    let nlist = recommended_nlist(EXPECTED_ROWS);
    println!("Recommended nlist for IVF_FLAT: {nlist}\n");

    // 2a. HNSW collection (low latency, high memory)
    let hnsw = create_collection_hnsw_body(COLLECTION, DIMS, metric_type::COSINE, 16, 200);
    println!("-- POST {} (HNSW collection) --", collections_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&hnsw).unwrap());

    // 2b. IVF_FLAT collection (lower memory, faster indexing)
    let ivf = create_collection_ivf_body(COLLECTION, DIMS, metric_type::L2, nlist);
    println!(
        "-- POST {} (IVF_FLAT collection) --",
        collections_url(&cfg.url)
    );
    println!("{}\n", serde_json::to_string_pretty(&ivf).unwrap());

    // 3. Partition strategy: shard by region
    let regions = ["us-east", "eu-west", "ap-south"];
    for region in &regions {
        let body = create_partition_body(COLLECTION, region);
        println!(
            "-- POST {} (partition: {region}) --",
            partitions_url(&cfg.url)
        );
        println!("{}\n", serde_json::to_string_pretty(&body).unwrap());
    }

    // 4. Load collection / partition
    let load_col = load_collection_body(COLLECTION);
    println!("-- POST {} --", collection_load_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&load_col).unwrap());

    let load_parts = load_partition_body(COLLECTION, &["us-east", "eu-west"]);
    println!("-- POST {} (selective) --", partition_load_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&load_parts).unwrap());

    // 5. Insert into partition
    let entities = vec![(
        vec![0.1f32; 8],
        serde_json::json!({"title": "example", "region": "us-east"}),
    )];
    let insert = insert_into_partition_body(COLLECTION, "us-east", &entities);
    println!(
        "-- POST {} (partition insert) --",
        entities_insert_url(&cfg.url)
    );
    println!("{}\n", serde_json::to_string_pretty(&insert).unwrap());

    // 6. Search with HNSW ef override (trading accuracy for speed)
    let fast_search = search_hnsw_body(COLLECTION, &[0.1f32; 8], 10, metric_type::COSINE, 32);
    println!("-- POST {} (hnsw ef=32) --", entities_search_url(&cfg.url));
    println!("{}\n", serde_json::to_string_pretty(&fast_search).unwrap());

    // 7. Partition-scoped search
    let part_search =
        search_partition_body(COLLECTION, "eu-west", &[0.1f32; 8], 10, metric_type::COSINE);
    println!(
        "-- POST {} (partition-scoped) --",
        entities_search_url(&cfg.url)
    );
    println!("{}\n", serde_json::to_string_pretty(&part_search).unwrap());

    // 8. Consistency level trade-offs
    for level in &[
        consistency::STRONG,
        consistency::BOUNDED,
        consistency::EVENTUALLY,
    ] {
        let b =
            search_with_consistency_body(COLLECTION, &[0.1f32; 8], 5, metric_type::COSINE, level);
        println!("-- Search with {level} consistency --");
        println!("{}\n", serde_json::to_string_pretty(&b).unwrap());
    }

    // 9. Hybrid dense+sparse search (requires sparse index on collection)
    let sparse = vec![(0u32, 0.9f32), (42u32, 0.4f32), (99u32, 0.2f32)];
    let hybrid = hybrid_search_body(
        COLLECTION,
        &[0.1f32; 8],
        "sparse_embedding",
        &sparse,
        10,
        metric_type::COSINE,
    );
    println!(
        "-- POST {} (hybrid dense+sparse) --",
        entities_hybrid_url(&cfg.url)
    );
    println!("{}\n", serde_json::to_string_pretty(&hybrid).unwrap());

    // 10. Delete stale data
    let del = delete_by_expr_body(COLLECTION, "created_at < 1700000000");
    println!(
        "-- POST {} (delete old data) --",
        entities_delete_url(&cfg.url)
    );
    println!("{}\n", serde_json::to_string_pretty(&del).unwrap());

    println!("Set MILVUS_URL to run against a live Milvus cluster.");
    println!("Index type comparison:");
    println!("  {} -- low-latency, memory-intensive", index_type::HNSW);
    println!("  {} -- balanced, disk-friendly", index_type::IVF_FLAT);
    println!("  {} -- auto-tuned by Milvus", index_type::AUTOINDEX);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hnsw_body_has_collection_name() {
        let body = create_collection_hnsw_body(COLLECTION, DIMS, metric_type::COSINE, 16, 200);
        assert_eq!(body["collectionName"], COLLECTION);
    }

    #[test]
    fn example_sizing_guidance_runs_without_panic() {
        let s = sizing_guidance(DIMS, EXPECTED_ROWS);
        assert!(s.len() > 0);
    }

    #[test]
    fn example_recommended_nlist_is_positive() {
        assert!(recommended_nlist(EXPECTED_ROWS) > 0);
    }
}
