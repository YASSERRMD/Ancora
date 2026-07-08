use crate::{
    api::{CostAnalytics, CostEvent},
    by_capability::Capability,
};

fn make_event(ts: u64, cost: f64, model: &str, provider: &str) -> CostEvent {
    CostEvent {
        timestamp: ts,
        cost_usd: cost,
        tokens: 200,
        model: model.to_string(),
        provider: provider.to_string(),
        tool: None,
        tenant_id: "tenant-x".to_string(),
        project_id: "proj-y".to_string(),
        capability: Capability::Generation,
        cache_hit: false,
        full_cost_if_miss: cost,
        actual_cache_cost: 0.0,
    }
}

fn make_event_with_tool(ts: u64, cost: f64, tool: &str) -> CostEvent {
    let mut e = make_event(ts, cost, "claude", "anthropic");
    e.tool = Some(tool.to_string());
    e
}

fn make_cache_hit_event(ts: u64, full_cost: f64, actual: f64) -> CostEvent {
    let mut e = make_event(ts, actual, "claude", "anthropic");
    e.cache_hit = true;
    e.full_cost_if_miss = full_cost;
    e.actual_cache_cost = actual;
    e
}

#[test]
fn api_accumulates_total_cost() {
    let mut a = CostAnalytics::new(3.0);
    a.ingest(make_event(100, 1.0, "claude", "anthropic"));
    a.ingest(make_event(200, 2.0, "gpt-4", "openai"));
    assert!((a.total_cost() - 3.0).abs() < 1e-9);
}

#[test]
fn api_top_models_ordered() {
    let mut a = CostAnalytics::new(3.0);
    a.ingest(make_event(100, 5.0, "expensive", "anthropic"));
    a.ingest(make_event(200, 1.0, "cheap", "anthropic"));
    let top = a.top_models();
    assert_eq!(top[0].0, "expensive");
}

#[test]
fn api_cache_hit_rate_tracked() {
    let mut a = CostAnalytics::new(3.0);
    a.ingest(make_cache_hit_event(100, 2.0, 0.2));
    a.ingest(make_event(200, 2.0, "claude", "anthropic")); // miss
    assert!((a.cache_hit_rate() - 0.5).abs() < 1e-9);
    assert!(a.cache_savings() > 0.0);
}

#[test]
fn api_tool_costs_tracked() {
    let mut a = CostAnalytics::new(3.0);
    a.ingest(make_event_with_tool(100, 0.5, "search"));
    a.ingest(make_event_with_tool(200, 0.3, "search"));
    a.ingest(make_event_with_tool(300, 0.1, "calculator"));
    let top = a.top_tools();
    assert_eq!(top[0].0, "search");
    assert!((top[0].1 - 0.8).abs() < 1e-9);
}

#[test]
fn api_snapshot_json_valid() {
    let mut a = CostAnalytics::new(3.0);
    a.ingest(make_event(100, 2.0, "claude", "anthropic"));
    let snap = a.snapshot("2025-06");
    let json = snap.to_json();
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
    assert!(json.contains("\"total_cost_usd\""));
}

#[test]
fn api_suggestions_generated() {
    let mut a = CostAnalytics::new(3.0);
    // Low cache hit rate and high model cost to trigger suggestions
    for i in 0u64..20 {
        a.ingest(make_event(i * 100, 10.0, "claude-opus", "anthropic"));
    }
    let suggestions = a.generate_suggestions();
    assert!(
        !suggestions.is_empty(),
        "should generate at least one suggestion"
    );
}
