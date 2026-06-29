/// Cost accounting per provider -- validates CostTracker records correctly.
/// Offline, no HTTP calls.
use ancora_core::cost::{CostTracker, TokenUsage};

// Approximate per-token rates (USD) for common providers.
const ANTHROPIC_IN: f64  = 0.000003;  // $3 per 1M
const ANTHROPIC_OUT: f64 = 0.000015;  // $15 per 1M
const OPENAI_IN: f64     = 0.000005;  // $5 per 1M
const OPENAI_OUT: f64    = 0.000015;  // $15 per 1M
const DEEPSEEK_IN: f64   = 0.0000002; // $0.2 per 1M
const DEEPSEEK_OUT: f64  = 0.0000002;
const QWEN_IN: f64       = 0.0000004;
const QWEN_OUT: f64      = 0.0000012;

fn usage(tokens_in: u64, tokens_out: u64) -> TokenUsage {
    TokenUsage { tokens_in, tokens_out }
}

#[test]
fn anthropic_cost_matches_rate() {
    let mut tracker = CostTracker::new(ANTHROPIC_IN, ANTHROPIC_OUT);
    tracker.record("draft", usage(1_000, 500));
    let summary = tracker.summary();
    let expected = 1_000.0 * ANTHROPIC_IN + 500.0 * ANTHROPIC_OUT;
    assert!((summary.total_cost_usd - expected).abs() < 1e-12);
}

#[test]
fn openai_cost_matches_rate() {
    let mut tracker = CostTracker::new(OPENAI_IN, OPENAI_OUT);
    tracker.record("plan", usage(2_000, 800));
    let summary = tracker.summary();
    let expected = 2_000.0 * OPENAI_IN + 800.0 * OPENAI_OUT;
    assert!((summary.total_cost_usd - expected).abs() < 1e-12);
}

#[test]
fn deepseek_is_cheaper_than_anthropic_same_tokens() {
    let mut ant = CostTracker::new(ANTHROPIC_IN, ANTHROPIC_OUT);
    ant.record("node", usage(1_000, 500));
    let mut ds = CostTracker::new(DEEPSEEK_IN, DEEPSEEK_OUT);
    ds.record("node", usage(1_000, 500));
    assert!(ds.summary().total_cost_usd < ant.summary().total_cost_usd);
}

#[test]
fn multi_node_cost_aggregates() {
    let mut tracker = CostTracker::new(OPENAI_IN, OPENAI_OUT);
    tracker.record("classify", usage(500, 100));
    tracker.record("plan",     usage(1_000, 400));
    tracker.record("classify", usage(200, 50));
    let summary = tracker.summary();
    let expected_total_in  = 500 + 1_000 + 200;
    let expected_total_out = 100 + 400 + 50;
    assert_eq!(summary.total_tokens_in, expected_total_in);
    assert_eq!(summary.total_tokens_out, expected_total_out);
    let expected_cost = expected_total_in as f64 * OPENAI_IN + expected_total_out as f64 * OPENAI_OUT;
    assert!((summary.total_cost_usd - expected_cost).abs() < 1e-12);
}

#[test]
fn zero_tokens_zero_cost() {
    let mut tracker = CostTracker::new(ANTHROPIC_IN, ANTHROPIC_OUT);
    tracker.record("silent", usage(0, 0));
    let summary = tracker.summary();
    assert_eq!(summary.total_cost_usd, 0.0);
}

#[test]
fn qwen_rates_applied_correctly() {
    let mut tracker = CostTracker::new(QWEN_IN, QWEN_OUT);
    tracker.record("node", usage(5_000, 2_000));
    let expected = 5_000.0 * QWEN_IN + 2_000.0 * QWEN_OUT;
    assert!((tracker.summary().total_cost_usd - expected).abs() < 1e-12);
}

#[test]
fn cost_summary_nodes_sorted() {
    let mut tracker = CostTracker::new(OPENAI_IN, OPENAI_OUT);
    tracker.record("z-node", usage(100, 50));
    tracker.record("a-node", usage(200, 80));
    let summary = tracker.summary();
    let nodes: Vec<&str> = summary.nodes.iter().map(|n| n.node_id.as_str()).collect();
    let mut sorted = nodes.clone();
    sorted.sort();
    assert_eq!(nodes, sorted, "CostSummary nodes must be sorted by node_id");
}

#[test]
fn total_tokens_in_sum_of_all_nodes() {
    let mut tracker = CostTracker::new(DEEPSEEK_IN, DEEPSEEK_OUT);
    tracker.record("a", usage(100, 30));
    tracker.record("b", usage(200, 60));
    tracker.record("c", usage(300, 90));
    let summary = tracker.summary();
    assert_eq!(summary.total_tokens_in, 600);
    assert_eq!(summary.total_tokens_out, 180);
}

#[test]
fn provider_with_free_output_tokens() {
    let mut tracker = CostTracker::new(0.000001, 0.0);
    tracker.record("node", usage(1_000, 50_000));
    let expected = 1_000.0 * 0.000001;
    assert!((tracker.summary().total_cost_usd - expected).abs() < 1e-12);
}
