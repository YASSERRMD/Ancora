use ancora_ageval::{
    CoordinationMetric, GuardrailMetric, MemoryMetric, PlanningMetric, ReasoningMetric,
    ReflectionMetric, RoutingMetric,
};

const EPS: f64 = 1e-9;

#[test]
fn all_seven_canonical_values() {
    assert!((PlanningMetric::score(&["a".into(), "b".into(), "c".into(), "d".into()], &["a".into(), "b".into(), "c".into()]) - 0.75).abs() < EPS);
    assert!((ReflectionMetric::score("short", "longer answer") - 1.0).abs() < EPS);
    assert!((ReflectionMetric::score("longer text here", "short") - 0.5).abs() < EPS);
    assert!((ReflectionMetric::score("x", "x") - 0.0).abs() < EPS);
    assert!((RoutingMetric::score(0.9, 300, 1000) - 0.8).abs() < EPS);
    assert!((RoutingMetric::score(0.85, 0, 1000) - 0.925).abs() < EPS);
    assert!((CoordinationMetric::score(3, 3) - 1.0).abs() < EPS);
    assert!((GuardrailMetric::score(1, 2) - 0.5).abs() < EPS);
    assert!((ReasoningMetric::score(4, 5) - 0.8).abs() < EPS);
    assert!((MemoryMetric::score(9, 10) - 0.9).abs() < EPS);
}
