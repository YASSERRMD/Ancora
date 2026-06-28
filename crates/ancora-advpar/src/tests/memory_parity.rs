use ancora_ageval::MemoryMetric;
use ancora_memcon::TokenBudget;

const EPS: f64 = 1e-9;

#[test]
fn memory_parity_nine_of_ten() {
    assert!((MemoryMetric::score(9, 10) - 0.9).abs() < EPS);
}

#[test]
fn memory_parity_perfect() {
    assert!((MemoryMetric::score(10, 10) - 1.0).abs() < EPS);
}

#[test]
fn memory_parity_zero() {
    assert!((MemoryMetric::score(0, 10) - 0.0).abs() < EPS);
}

#[test]
fn memory_parity_empty() {
    assert!((MemoryMetric::score(0, 0) - 1.0).abs() < EPS);
}

#[test]
fn memory_parity_token_estimate() {
    // 1 token per 4 chars: "test" (4 chars) = 1 token
    let s = "test";
    assert_eq!(TokenBudget::estimate_tokens(s), 1);
}

#[test]
fn memory_parity_token_total() {
    let contents = vec!["aaaa".to_string(), "bbbb".to_string()]; // 4+4 chars = 2 tokens
    assert_eq!(TokenBudget::total_tokens(&contents), 2);
}
