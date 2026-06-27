use ancora_core::cost::{CostTracker, TokenUsage};

fn usage(tokens_in: u64, tokens_out: u64) -> TokenUsage {
    TokenUsage { tokens_in, tokens_out }
}

#[test]
fn empty_tracker_produces_zero_cost_summary() {
    let tracker = CostTracker::new(0.001, 0.002);
    let summary = tracker.summary();
    assert_eq!(summary.total_tokens_in, 0);
    assert_eq!(summary.total_tokens_out, 0);
    assert_eq!(summary.total_cost_usd, 0.0);
    assert!(summary.nodes.is_empty());
}

#[test]
fn single_node_cost_computed_correctly() {
    let mut tracker = CostTracker::new(0.001, 0.002);
    tracker.record("node-a", usage(100, 50));
    let summary = tracker.summary();

    assert_eq!(summary.total_tokens_in, 100);
    assert_eq!(summary.total_tokens_out, 50);

    let expected = 100.0 * 0.001 + 50.0 * 0.002;
    let node = summary.nodes.iter().find(|n| n.node_id == "node-a").unwrap();
    assert!((node.cost_usd - expected).abs() < 1e-9, "cost must match exact formula");
    assert!((summary.total_cost_usd - expected).abs() < 1e-9);
}

#[test]
fn two_nodes_aggregated_independently() {
    let mut tracker = CostTracker::new(0.001, 0.002);
    tracker.record("node-a", usage(100, 50));
    tracker.record("node-b", usage(200, 100));
    let summary = tracker.summary();

    assert_eq!(summary.total_tokens_in, 300);
    assert_eq!(summary.total_tokens_out, 150);
    assert_eq!(summary.nodes.len(), 2);
}

#[test]
fn repeated_records_for_same_node_accumulate() {
    let mut tracker = CostTracker::new(0.001, 0.002);
    tracker.record("node-a", usage(50, 25));
    tracker.record("node-a", usage(50, 25));
    let summary = tracker.summary();

    assert_eq!(summary.total_tokens_in, 100);
    assert_eq!(summary.total_tokens_out, 50);
    assert_eq!(summary.nodes.len(), 1, "same node must produce one NodeCost");
}

#[test]
fn nodes_sorted_by_id_in_summary() {
    let mut tracker = CostTracker::new(0.001, 0.002);
    tracker.record("z-node", usage(10, 10));
    tracker.record("a-node", usage(10, 10));
    tracker.record("m-node", usage(10, 10));
    let summary = tracker.summary();

    let ids: Vec<&str> = summary.nodes.iter().map(|n| n.node_id.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "nodes must be sorted by id");
}

#[test]
fn zero_price_produces_zero_cost() {
    let mut tracker = CostTracker::new(0.0, 0.0);
    tracker.record("node", usage(999, 999));
    let summary = tracker.summary();
    assert_eq!(summary.total_cost_usd, 0.0);
}

#[test]
fn cost_aggregation_is_accurate_for_large_counts() {
    let mut tracker = CostTracker::new(0.000_001, 0.000_002);
    let n = 1_000u64;
    for _ in 0..n {
        tracker.record("node", usage(1_000, 500));
    }
    let summary = tracker.summary();

    assert_eq!(summary.total_tokens_in, n * 1_000);
    assert_eq!(summary.total_tokens_out, n * 500);

    let expected = (n * 1_000) as f64 * 0.000_001 + (n * 500) as f64 * 0.000_002;
    assert!(
        (summary.total_cost_usd - expected).abs() < 1e-6,
        "cost must be accurate for large token counts"
    );
}

#[test]
fn node_cost_sum_equals_total_cost() {
    let mut tracker = CostTracker::new(0.01, 0.02);
    for i in 0..5 {
        tracker.record(&format!("n{}", i), usage(100, 50));
    }
    let summary = tracker.summary();
    let node_sum: f64 = summary.nodes.iter().map(|n| n.cost_usd).sum();
    assert!((node_sum - summary.total_cost_usd).abs() < 1e-9, "node sum must equal total");
}
