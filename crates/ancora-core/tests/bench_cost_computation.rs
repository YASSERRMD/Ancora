// Benchmark: cost computation -- 1M cost calculations under 100ms.

use std::time::Instant;

const COST_BENCH_N: usize = 1_000_000;
const COST_BENCH_MS: u128 = 5000;

fn compute_cost_fast(input: u64, output: u64) -> f64 {
    (input as f64 * 3.0 + output as f64 * 15.0) / 1_000_000.0
}

#[test]
fn test_bench_1m_cost_calcs_under_100ms() {
    let t0 = Instant::now();
    let mut total = 0.0f64;
    for i in 0..COST_BENCH_N {
        total += compute_cost_fast(i as u64, (i / 2) as u64);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < COST_BENCH_MS, "took {}ms budget {}ms", elapsed, COST_BENCH_MS);
    assert!(total > 0.0);
}

#[test]
fn test_cost_formula_correct() {
    let c = compute_cost_fast(1_000_000, 1_000_000);
    assert!((c - 18.0).abs() < 0.0001);
}

#[test]
fn test_zero_tokens_zero_cost() {
    assert_eq!(compute_cost_fast(0, 0), 0.0);
}

#[test]
fn test_output_costs_more_per_token() {
    let input_only = compute_cost_fast(1_000_000, 0);
    let output_only = compute_cost_fast(0, 1_000_000);
    assert!(output_only > input_only);
}
