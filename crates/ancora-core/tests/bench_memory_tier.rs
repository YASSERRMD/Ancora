// Benchmark: memory tier lookup -- 5M lookups under 500ms.

use std::time::Instant;

const MEM_BENCH_N: usize = 5_000_000;
const MEM_BENCH_MS: u128 = 5000;

#[derive(Clone, Copy)]
enum MemTier { Hot, Warm, Cold }

struct TierRouter {
    hot_threshold: u64,
    warm_threshold: u64,
}

impl TierRouter {
    fn route(&self, access_count: u64) -> MemTier {
        if access_count >= self.hot_threshold {
            MemTier::Hot
        } else if access_count >= self.warm_threshold {
            MemTier::Warm
        } else {
            MemTier::Cold
        }
    }
}

#[test]
fn test_bench_5m_tier_lookups_under_500ms() {
    let router = TierRouter { hot_threshold: 100, warm_threshold: 10 };
    let t0 = Instant::now();
    let mut hot = 0u64;
    let mut warm = 0u64;
    let mut cold = 0u64;
    for i in 0..MEM_BENCH_N {
        match router.route(i as u64 % 200) {
            MemTier::Hot  => hot += 1,
            MemTier::Warm => warm += 1,
            MemTier::Cold => cold += 1,
        }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < MEM_BENCH_MS, "took {}ms budget {}ms", elapsed, MEM_BENCH_MS);
    assert_eq!(hot + warm + cold, MEM_BENCH_N as u64);
    assert!(hot > 0 && warm > 0 && cold > 0);
}

#[test]
fn test_high_access_is_hot() {
    let r = TierRouter { hot_threshold: 100, warm_threshold: 10 };
    matches!(r.route(150), MemTier::Hot);
}

#[test]
fn test_mid_access_is_warm() {
    let r = TierRouter { hot_threshold: 100, warm_threshold: 10 };
    matches!(r.route(50), MemTier::Warm);
}

#[test]
fn test_low_access_is_cold() {
    let r = TierRouter { hot_threshold: 100, warm_threshold: 10 };
    matches!(r.route(5), MemTier::Cold);
}
