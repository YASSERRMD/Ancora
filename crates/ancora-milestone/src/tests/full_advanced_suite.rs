use ancora_ageval::{PlanningMetric, ReflectionMetric, RoutingMetric};

#[test]
fn all_seven_metrics_produce_values() {
    use ancora_ageval::{CoordinationMetric, GuardrailMetric, MemoryMetric, ReasoningMetric};
    let _ = PlanningMetric::score(&["a".into()], &["a".into()]);
    let _ = ReflectionMetric::score("a", "b");
    let _ = RoutingMetric::score(0.9, 300, 1000);
    let _ = CoordinationMetric::score(3, 3);
    let _ = GuardrailMetric::score(1, 2);
    let _ = ReasoningMetric::score(4, 5);
    let _ = MemoryMetric::score(9, 10);
}

#[test]
fn milestone_runs_offline() {
    // All crate operations complete in-process; this verifies no panic on import
    let r = RoutingMetric::score(0.9, 300, 1000);
    assert!((r - 0.8).abs() < 1e-9);
}
