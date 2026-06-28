use ancora_ageval::{PlanningMetric, ReflectionMetric, RoutingMetric};

const EPS: f64 = 1e-12;

#[test]
fn planning_score_same_5_runs() {
    let expected: Vec<String> = (0..100).map(|i| format!("s{i}")).collect();
    let actual: Vec<String> = (0..75).map(|i| format!("s{i}")).collect();
    let first = PlanningMetric::score(&expected, &actual);
    for _ in 0..4 {
        let again = PlanningMetric::score(&expected, &actual);
        assert!((again - first).abs() < EPS, "planning score not reproducible");
    }
}

#[test]
fn reflection_score_same_5_runs() {
    let first = ReflectionMetric::score("abc", "abcdef");
    for _ in 0..4 {
        let again = ReflectionMetric::score("abc", "abcdef");
        assert!((again - first).abs() < EPS, "reflection score not reproducible");
    }
}

#[test]
fn routing_score_same_5_runs() {
    let first = RoutingMetric::score(0.9, 300, 1000);
    for _ in 0..4 {
        let again = RoutingMetric::score(0.9, 300, 1000);
        assert!((again - first).abs() < EPS, "routing score not reproducible");
    }
}
