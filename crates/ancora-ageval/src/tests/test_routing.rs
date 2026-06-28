use crate::routing::RoutingMetric;

#[test]
fn routing_metric_on_fixture() {
    // low cost (10/100 = 0.1) + high quality (0.9) -> good score
    let score = RoutingMetric::score(0.9, 10, 100);
    let expected = (0.9 + 0.9) / 2.0;
    assert!((score - expected).abs() < 1e-10);
}

#[test]
fn routing_high_cost_penalized() {
    // max cost + good quality -> penalized score
    let score = RoutingMetric::score(1.0, 100, 100);
    // cost_efficiency = 0, so (1.0 + 0.0) / 2 = 0.5
    assert!((score - 0.5).abs() < 1e-10);
}

#[test]
fn routing_zero_max_cost_returns_quality() {
    let score = RoutingMetric::score(0.75, 0, 0);
    assert_eq!(score, 0.75);
}
