// Cross-language parity: canonical numeric values that must match in all language ports.
use ancora_ageval::{
    CoordinationMetric, GuardrailMetric, MemoryMetric, PlanningMetric, ReasoningMetric,
    ReflectionMetric, RoutingMetric,
};

#[test]
fn parity_planning_three_of_four() {
    let expected: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
    let actual: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let score = PlanningMetric::score(&expected, &actual);
    assert!((score - 0.75).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_reflection_improved() {
    let score = ReflectionMetric::score("short", "this is a longer improved answer");
    assert!((score - 1.0).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_routing_canonical() {
    // quality=0.9, cost=300, max_cost=1000 -> efficiency=0.7 -> blended=0.8
    let score = RoutingMetric::score(0.9, 300, 1000);
    assert!((score - 0.8).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_coordination_perfect() {
    let score = CoordinationMetric::score(3, 3);
    assert!((score - 1.0).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_guardrail_half() {
    let score = GuardrailMetric::score(1, 2);
    assert!((score - 0.5).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_reasoning_four_of_five() {
    let score = ReasoningMetric::score(4, 5);
    assert!((score - 0.8).abs() < f64::EPSILON, "got {score}");
}

#[test]
fn parity_memory_nine_of_ten() {
    let score = MemoryMetric::score(9, 10);
    assert!((score - 0.9).abs() < f64::EPSILON, "got {score}");
}
