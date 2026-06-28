use ancora_ageval::RoutingMetric;

const EPS: f64 = 1e-9;

#[test]
fn routing_parity_canonical() {
    // quality=0.9, cost=300, max=1000 -> efficiency=0.7 -> (0.9+0.7)/2 = 0.8
    assert!((RoutingMetric::score(0.9, 300, 1000) - 0.8).abs() < EPS);
}

#[test]
fn routing_parity_zero_cost() {
    // quality=0.85, cost=0, max=1000 -> efficiency=1.0 -> (0.85+1.0)/2 = 0.925
    assert!((RoutingMetric::score(0.85, 0, 1000) - 0.925).abs() < EPS);
}

#[test]
fn routing_parity_max_cost() {
    // quality=0.8, cost=1000, max=1000 -> efficiency=0.0 -> 0.4
    assert!((RoutingMetric::score(0.8, 1000, 1000) - 0.4).abs() < EPS);
}

#[test]
fn routing_parity_zero_max_cost() {
    // max_cost=0 -> returns quality unchanged
    assert!((RoutingMetric::score(0.7, 500, 0) - 0.7).abs() < EPS);
}

#[test]
fn routing_parity_perfect_score() {
    // quality=1.0, cost=0, max=100 -> 1.0
    assert!((RoutingMetric::score(1.0, 0, 100) - 1.0).abs() < EPS);
}
