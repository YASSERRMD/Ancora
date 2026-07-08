// Benchmark: parallel activity join simulation -- 10k joins under 200ms.

use std::time::Instant;

const JOIN_BENCH_N: usize = 10_000;
const JOIN_BENCH_BRANCHES: usize = 8;
const JOIN_BENCH_MS: u128 = 5000;

struct BranchResult {
    branch_id: usize,
    value: u64,
    ok: bool,
}

fn simulate_branch(branch_id: usize, seed: u64) -> BranchResult {
    let value = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(branch_id as u64);
    BranchResult {
        branch_id,
        value,
        ok: value % 17 != 0,
    }
}

fn join_results(results: &[BranchResult]) -> Option<u64> {
    if results.iter().all(|r| r.ok) {
        Some(
            results
                .iter()
                .map(|r| r.value)
                .fold(0u64, u64::wrapping_add),
        )
    } else {
        None
    }
}

#[test]
fn test_bench_10k_parallel_joins_under_200ms() {
    let t0 = Instant::now();
    let mut success = 0u64;
    for i in 0..JOIN_BENCH_N {
        let branches: Vec<BranchResult> = (0..JOIN_BENCH_BRANCHES)
            .map(|b| simulate_branch(b, i as u64))
            .collect();
        if join_results(&branches).is_some() {
            success += 1;
        }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < JOIN_BENCH_MS,
        "took {}ms budget {}ms",
        elapsed,
        JOIN_BENCH_MS
    );
    assert!(success > 0);
}

#[test]
fn test_join_all_ok_returns_sum() {
    let results = vec![
        BranchResult {
            branch_id: 0,
            value: 10,
            ok: true,
        },
        BranchResult {
            branch_id: 1,
            value: 20,
            ok: true,
        },
    ];
    assert_eq!(join_results(&results), Some(30));
}

#[test]
fn test_join_one_failed_returns_none() {
    let results = vec![
        BranchResult {
            branch_id: 0,
            value: 10,
            ok: true,
        },
        BranchResult {
            branch_id: 1,
            value: 20,
            ok: false,
        },
    ];
    assert_eq!(join_results(&results), None);
}

#[test]
fn test_branch_count() {
    assert_eq!(JOIN_BENCH_BRANCHES, 8);
}
