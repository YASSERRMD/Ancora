use ancora_ageval::ReflectionMetric;

#[test]
fn reflection_score_stable_on_growth() {
    let s1 = ReflectionMetric::score("short", "longer answer here");
    let s2 = ReflectionMetric::score("short", "longer answer here");
    assert!((s1 - s2).abs() < f64::EPSILON);
    assert!((s1 - 1.0).abs() < f64::EPSILON);
}

#[test]
fn reflection_score_stable_on_shrink() {
    let s1 = ReflectionMetric::score("a long string here", "shorter");
    let s2 = ReflectionMetric::score("a long string here", "shorter");
    assert!((s1 - s2).abs() < f64::EPSILON);
    assert!((s1 - 0.5).abs() < f64::EPSILON);
}

#[test]
fn reflection_score_stable_unchanged() {
    let s1 = ReflectionMetric::score("same", "same");
    let s2 = ReflectionMetric::score("same", "same");
    assert!((s1 - s2).abs() < f64::EPSILON);
    assert_eq!(s1, 0.0);
}
