use ancora_ageval::PlanningMetric;

fn steps() -> (Vec<String>, Vec<String>) {
    let expected = vec!["search".into(), "summarize".into(), "respond".into(), "review".into()];
    let actual = vec!["search".into(), "summarize".into(), "respond".into()];
    (expected, actual)
}

#[test]
fn optimization_artifact_score_stable() {
    let (exp, act) = steps();
    let s1 = PlanningMetric::score(&exp, &act);
    let s2 = PlanningMetric::score(&exp, &act);
    assert!((s1 - s2).abs() < f64::EPSILON);
}

#[test]
fn optimization_artifact_canonical_value() {
    let (exp, act) = steps();
    let score = PlanningMetric::score(&exp, &act);
    // 3 matched out of 4 expected
    assert!((score - 0.75).abs() < f64::EPSILON);
}

#[test]
fn optimization_artifact_full_match_stable() {
    let steps: Vec<String> = vec!["a".into(), "b".into()];
    let s1 = PlanningMetric::score(&steps, &steps);
    let s2 = PlanningMetric::score(&steps, &steps);
    assert!((s1 - s2).abs() < f64::EPSILON);
    assert!((s1 - 1.0).abs() < f64::EPSILON);
}
