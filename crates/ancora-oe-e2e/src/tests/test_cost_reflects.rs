use crate::cost_e2e::{simulate_run_cost, CostEntry, CostReport, TokenUsage};

#[test]
fn cost_analytics_reflect_a_run() {
    let report = simulate_run_cost("run-cost-001");

    assert!(!report.is_empty(), "cost report must not be empty");
    assert!(report.total_tokens() > 0, "total tokens must be positive");
    assert!(
        report.total_cost_microdollars() > 0,
        "total cost must be positive"
    );
}

#[test]
fn cost_report_aggregates_by_model() {
    let report = simulate_run_cost("run-cost-002");
    let by_model = report.by_model();

    assert!(!by_model.is_empty(), "model breakdown must not be empty");
    // All entries in default simulation use local-judge.
    assert!(by_model.contains_key("local-judge"));
}

#[test]
fn cost_entry_dollars_conversion_is_correct() {
    let usage = TokenUsage::new(100, 50);
    let entry = CostEntry::new("run-x", "model-a", usage, 1_000_000);
    let dollars = entry.cost_dollars();
    assert!(
        (dollars - 1.0).abs() < f64::EPSILON,
        "1_000_000 microdollars must equal $1.00"
    );
}

#[test]
fn token_usage_total_is_sum() {
    let usage = TokenUsage::new(300, 100);
    assert_eq!(usage.total(), 400);
}

#[test]
fn empty_report_has_zero_totals() {
    let report = CostReport::new();
    assert_eq!(report.total_tokens(), 0);
    assert_eq!(report.total_cost_microdollars(), 0);
}
