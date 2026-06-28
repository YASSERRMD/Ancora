// Benchmark: policy evaluation -- 2M checks under 500ms.

use std::time::Instant;

const POLICY_BENCH_N: usize = 2_000_000;
const POLICY_BENCH_MS: u128 = 500;

const ALLOWED_MODELS: &[&str] = &[
    "claude-3-5-haiku",
    "claude-3-5-sonnet",
    "qwen3-local",
];

fn policy_check_model(model: &str) -> bool {
    ALLOWED_MODELS.contains(&model)
}

fn policy_check_cost(accumulated_usd: f64, ceiling: f64) -> bool {
    accumulated_usd <= ceiling
}

#[test]
fn test_bench_2m_policy_checks_under_500ms() {
    let models = ["claude-3-5-haiku", "gpt-4o", "claude-3-5-sonnet", "gemini-pro", "qwen3-local", "llama3"];
    let t0 = Instant::now();
    let mut allowed = 0u64;
    for i in 0..POLICY_BENCH_N {
        let model = models[i % 6];
        if policy_check_model(model) { allowed += 1; }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(elapsed < POLICY_BENCH_MS, "took {}ms budget {}ms", elapsed, POLICY_BENCH_MS);
    // 3 of 6 models allowed
    assert_eq!(allowed, (POLICY_BENCH_N / 6 * 3 + POLICY_BENCH_N % 6) as u64, "approx");
}

#[test]
fn test_allowed_model_passes() {
    assert!(policy_check_model("claude-3-5-haiku"));
    assert!(policy_check_model("qwen3-local"));
}

#[test]
fn test_blocked_model_fails() {
    assert!(!policy_check_model("gpt-4o"));
}

#[test]
fn test_cost_within_ceiling_passes() {
    assert!(policy_check_cost(4.99, 5.0));
}

#[test]
fn test_cost_over_ceiling_fails() {
    assert!(!policy_check_cost(5.01, 5.0));
}
