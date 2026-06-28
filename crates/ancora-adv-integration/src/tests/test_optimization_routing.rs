use ancora_ageval::{BaselineStore, EvalReport, MetricScore, PlanningMetric, RoutingMetric};

#[test]
fn optimization_plus_routing() {
    // Simulate two routing options and select the better one via eval metrics
    let option_a = RoutingMetric::score(0.9, 15, 100); // high quality, low cost
    let option_b = RoutingMetric::score(0.7, 5, 100);  // lower quality, even lower cost

    // Option A wins on the combined metric
    assert!(option_a > option_b);

    // Planning quality with the selected route
    let plan = PlanningMetric::score(
        &["select-route".into(), "execute".into()],
        &["select-route".into(), "execute".into()],
    );
    assert_eq!(plan, 1.0);
}

#[test]
fn regression_baseline_integrates_with_report() {
    let mut store = BaselineStore::new(0.05);
    store.set("routing_cost_quality", 0.8);

    let current_score = RoutingMetric::score(0.9, 20, 100);
    let result = store.check("routing_cost_quality", current_score);
    assert!(matches!(result, ancora_ageval::BaselineResult::Passed { .. }));

    let mut report = EvalReport::new("opt-routing", 1);
    report.add_score(MetricScore::new("routing_cost_quality", current_score));
    assert!(!report.has_regressions());
    assert!(report.mean_score() > 0.0);
}
