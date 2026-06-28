use ancora_ageval::{PlanningMetric, ReflectionMetric, RoutingMetric};

const EPS: f64 = 1e-9;

#[test]
fn parity_planning_3_of_4() {
    let s = PlanningMetric::score(
        &["a".into(), "b".into(), "c".into(), "d".into()],
        &["a".into(), "b".into(), "c".into()],
    );
    assert!((s - 0.75).abs() < EPS);
}

#[test]
fn parity_reflection_grew() {
    assert!((ReflectionMetric::score("short", "longer answer") - 1.0).abs() < EPS);
}

#[test]
fn parity_routing_canonical() {
    assert!((RoutingMetric::score(0.9, 300, 1000) - 0.8).abs() < EPS);
}
