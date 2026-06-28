use ancora_ageval::{PlanningMetric, ReflectionMetric};

#[test]
fn planning_deterministic_across_5_runs() {
    let exp: Vec<String> = (0..50).map(|i| format!("s{i}")).collect();
    let act: Vec<String> = (0..40).map(|i| format!("s{i}")).collect();
    let first = PlanningMetric::score(&exp, &act);
    for _ in 0..4 {
        assert_eq!(PlanningMetric::score(&exp, &act), first);
    }
}

#[test]
fn reflection_deterministic_across_5_runs() {
    let first = ReflectionMetric::score("abc", "abcdefgh");
    for _ in 0..4 {
        assert_eq!(ReflectionMetric::score("abc", "abcdefgh"), first);
    }
}
