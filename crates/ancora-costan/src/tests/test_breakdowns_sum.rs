use crate::{
    by_model::ModelCostBreakdown,
    by_provider::ProviderCostBreakdown,
    by_tool::ToolCostBreakdown,
    by_tenant::TenantCostBreakdown,
    by_capability::{Capability, CapabilityCostBreakdown},
};

#[test]
fn model_breakdown_sums_to_total() {
    let mut b = ModelCostBreakdown::new();
    b.record("gpt-4", 3.0, 3000);
    b.record("claude-3", 2.0, 2000);
    b.record("mistral", 1.0, 1000);
    let from_top: f64 = b.top_models().iter().map(|(_, c)| c).sum();
    assert!((from_top - b.total_cost()).abs() < 1e-9,
        "top models sum {} != total {}", from_top, b.total_cost());
}

#[test]
fn provider_breakdown_sums_to_total() {
    let mut b = ProviderCostBreakdown::new();
    b.record("anthropic", 4.0);
    b.record("openai", 3.0);
    b.record("mistral", 1.5);
    let from_top: f64 = b.top_providers().iter().map(|(_, c)| c).sum();
    assert!((from_top - b.total_cost()).abs() < 1e-9);
}

#[test]
fn tool_breakdown_sums_to_total() {
    let mut b = ToolCostBreakdown::new();
    b.record("search", 2.0);
    b.record("calculator", 0.5);
    b.record("code_runner", 1.5);
    let from_top: f64 = b.top_tools_by_cost().iter().map(|(_, c)| c).sum();
    assert!((from_top - b.total_cost()).abs() < 1e-9);
}

#[test]
fn tenant_breakdown_sums_to_total() {
    let mut b = TenantCostBreakdown::new();
    b.record("tenant-a", "proj-1", 2.0);
    b.record("tenant-a", "proj-2", 1.0);
    b.record("tenant-b", "proj-1", 3.0);
    let from_top: f64 = b.top_tenants().iter().map(|(_, c)| c).sum();
    assert!((from_top - b.total_cost()).abs() < 1e-9);
}

#[test]
fn capability_fractions_sum_to_one() {
    let mut b = CapabilityCostBreakdown::new();
    b.record(Capability::Planner, 1.0);
    b.record(Capability::Reflection, 2.0);
    b.record(Capability::Routing, 1.0);
    let total = b.total_cost();
    let sum_fracs =
        b.fraction_for(&Capability::Planner)
        + b.fraction_for(&Capability::Reflection)
        + b.fraction_for(&Capability::Routing);
    assert!((sum_fracs - 1.0).abs() < 1e-9,
        "fractions sum to {} not 1.0 (total={})", sum_fracs, total);
}
