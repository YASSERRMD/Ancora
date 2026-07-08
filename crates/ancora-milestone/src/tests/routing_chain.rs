use ancora_ageval::RoutingMetric;

const EPS: f64 = 1e-9;

#[test]
fn routing_chain_canonical_score() {
    let score = RoutingMetric::score(0.9, 300, 1000);
    assert!((score - 0.8).abs() < EPS);
}

#[test]
fn routing_chain_zero_cost_gives_best_score() {
    let with_cost = RoutingMetric::score(0.9, 500, 1000);
    let no_cost = RoutingMetric::score(0.9, 0, 1000);
    assert!(no_cost > with_cost, "zero cost should score higher");
}

#[test]
fn routing_chain_scores_in_range() {
    for cost in [0u64, 100, 500, 900, 1000] {
        let s = RoutingMetric::score(1.0, cost, 1000);
        assert!(
            (0.0..=1.0).contains(&s),
            "score {s} out of range for cost={cost}"
        );
    }
}
