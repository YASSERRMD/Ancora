use crate::dashboard::{DashboardBuilder, DashboardSnapshot};

#[test]
fn dashboard_json_is_valid_object() {
    let mut builder = DashboardBuilder::new("2025-01");
    builder.timeseries_mut().record(1000, 3.0, 300);
    builder.timeseries_mut().record(2000, 2.0, 200);
    builder.model_mut().record("claude-3", 3.0, 300);
    builder.model_mut().record("gpt-4", 2.0, 200);
    builder.provider_mut().record("anthropic", 3.0);
    builder.provider_mut().record("openai", 2.0);
    let snap = builder.build();
    let json = snap.to_json();

    assert!(json.starts_with('{'), "JSON should start with {{");
    assert!(json.ends_with('}'), "JSON should end with }}");
    assert!(
        json.contains("\"total_cost_usd\""),
        "JSON missing total_cost_usd"
    );
    assert!(json.contains("\"period\""), "JSON missing period");
    assert!(json.contains("\"top_models\""), "JSON missing top_models");
    assert!(
        json.contains("\"top_providers\""),
        "JSON missing top_providers"
    );
    assert!(
        json.contains("\"cache_hit_rate\""),
        "JSON missing cache_hit_rate"
    );
}

#[test]
fn dashboard_total_cost_correct() {
    let mut builder = DashboardBuilder::new("2025-06");
    builder.timeseries_mut().record(1000, 5.0, 500);
    builder.timeseries_mut().record(2000, 3.0, 300);
    let snap = builder.build();
    assert!((snap.total_cost_usd - 8.0).abs() < 1e-9);
}

#[test]
fn dashboard_cache_hit_rate_reflected() {
    let mut builder = DashboardBuilder::new("2025-06");
    builder.timeseries_mut().record(1000, 1.0, 100);
    builder.cache_mut().record_hit(2.0, 0.2, 100);
    builder.cache_mut().record_miss(1.0, 100);
    let snap = builder.build();
    let json = snap.to_json();
    assert!(json.contains("cache_hit_rate"));
    // hit_rate: 1 hit / 2 total = 0.5
    assert!((snap.cache_hit_rate - 0.5).abs() < 1e-9);
}

#[test]
fn empty_dashboard_json_valid() {
    let snap = DashboardSnapshot::default();
    let json = snap.to_json();
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
}
