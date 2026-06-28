use ancora_ageval::RoutingMetric;

#[test]
fn routing_score_stable() {
    let s1 = RoutingMetric::score(0.9, 300, 1000);
    let s2 = RoutingMetric::score(0.9, 300, 1000);
    assert!((s1 - s2).abs() < f64::EPSILON);
}

#[test]
fn routing_score_canonical_value() {
    let score = RoutingMetric::score(0.9, 300, 1000);
    // cost_efficiency = 1 - 0.3 = 0.7; blended = (0.9 + 0.7) / 2 = 0.8
    assert!((score - 0.8).abs() < f64::EPSILON);
}

#[test]
fn routing_score_zero_cost_stable() {
    let s1 = RoutingMetric::score(0.85, 0, 1000);
    let s2 = RoutingMetric::score(0.85, 0, 1000);
    assert!((s1 - s2).abs() < f64::EPSILON);
    // cost_efficiency = 1.0; blended = (0.85 + 1.0) / 2 = 0.925
    assert!((s1 - 0.925).abs() < f64::EPSILON);
}
