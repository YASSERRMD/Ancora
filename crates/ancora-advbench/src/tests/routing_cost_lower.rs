use ancora_ageval::RoutingMetric;

const EPS: f64 = 1e-9;

#[test]
fn routing_with_cost_lower_than_no_cost() {
    // high quality, no cost: pure quality score
    let no_cost = RoutingMetric::score(0.9, 0, 1000);
    // high quality, some cost
    let with_cost = RoutingMetric::score(0.9, 300, 1000);
    assert!(
        with_cost < no_cost,
        "using cost should reduce score: {with_cost} >= {no_cost}"
    );
}

#[test]
fn routing_score_canonical_0_9_300() {
    let score = RoutingMetric::score(0.9, 300, 1000);
    assert!(
        (score - 0.8).abs() < EPS,
        "canonical routing score should be 0.8, got {score}"
    );
}

#[test]
fn routing_cost_savings_at_zero_cost() {
    let score = RoutingMetric::score(0.85, 0, 1000);
    assert!(
        (score - 0.925).abs() < EPS,
        "zero cost routing should be 0.925, got {score}"
    );
}
