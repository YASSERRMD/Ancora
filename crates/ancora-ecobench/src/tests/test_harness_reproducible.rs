//! Tests: harness produces reproducible and internally consistent results.

use crate::harness::{run_bench, BenchConfig};

#[test]
fn harness_sample_count_matches_config() {
    let cfg = BenchConfig::new("reproducible").with_warmup(2).with_iters(8);
    let stats = run_bench(&cfg, || {
        // Trivial, deterministic work.
        let _: u64 = (0u64..100).sum();
    });
    assert_eq!(stats.sample_count, 8);
}

#[test]
fn harness_min_le_mean_le_max() {
    let cfg = BenchConfig::new("ordering").with_warmup(0).with_iters(10);
    let stats = run_bench(&cfg, || {
        let _v: Vec<u8> = (0..32).collect();
    });
    assert!(stats.min <= stats.mean, "min must be <= mean");
    assert!(stats.mean <= stats.max, "mean must be <= max");
}

#[test]
fn harness_name_preserved() {
    let cfg = BenchConfig::new("my-unique-bench-name");
    let stats = run_bench(&cfg, || {});
    assert_eq!(stats.name, "my-unique-bench-name");
}

#[test]
fn harness_zero_warmup_still_works() {
    let cfg = BenchConfig::new("no-warmup").with_warmup(0).with_iters(3);
    let stats = run_bench(&cfg, || {
        let _ = "hello".to_uppercase();
    });
    assert_eq!(stats.sample_count, 3);
    assert!(stats.min <= stats.max);
}
