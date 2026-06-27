/// Tests for Milvus collection sizing guidance helpers.
/// All offline.

#[cfg(test)]
mod milvus_sizing_tests {
    use crate::backends::milvus::*;

    #[test]
    fn sizing_guidance_contains_all_fields() {
        let s = sizing_guidance(128, 1_000_000);
        assert!(s.contains("dims=128"), "s: {s}");
        assert!(s.contains("rows=1000000"), "s: {s}");
        assert!(s.contains("raw_vector_mb="), "s: {s}");
        assert!(s.contains("recommended_nlist="), "s: {s}");
    }

    #[test]
    fn raw_vector_mb_matches_expected() {
        // 1M rows * 128 dims * 4 bytes = 512_000_000 bytes = 488 MiB (integer division)
        let s = sizing_guidance(128, 1_000_000);
        assert!(s.contains("raw_vector_mb=488"), "s: {s}");
    }

    #[test]
    fn recommended_nlist_for_small_set_is_at_least_one() {
        assert!(recommended_nlist(1) >= 1);
    }

    #[test]
    fn recommended_nlist_for_1m_rows_is_reasonable() {
        let n = recommended_nlist(1_000_000);
        // sqrt(1M) = 1000
        assert!(n >= 500 && n <= 4_000, "nlist={n}");
    }

    #[test]
    fn recommended_nlist_caps_at_65536() {
        let n = recommended_nlist(u64::MAX);
        assert!(n <= 65536, "nlist={n}");
    }

    #[test]
    fn sizing_guidance_zero_rows_does_not_panic() {
        let s = sizing_guidance(128, 0);
        assert!(s.contains("rows=0"), "s: {s}");
    }

    #[test]
    fn nlist_grows_with_row_count() {
        let small = recommended_nlist(10_000);
        let large = recommended_nlist(100_000_000);
        assert!(large > small, "nlist should grow with dataset size");
    }

    #[test]
    fn ivf_collection_uses_recommended_nlist() {
        let nlist = recommended_nlist(10_000_000);
        let body = create_collection_ivf_body("docs", 128, metric_type::L2, nlist);
        assert_eq!(body["indexParams"][0]["params"]["nlist"].as_u64().unwrap(), nlist as u64);
    }
}
