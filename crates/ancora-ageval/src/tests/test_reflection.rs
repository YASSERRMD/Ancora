use crate::reflection::ReflectionMetric;

#[test]
fn reflection_metric_on_fixture() {
    let score = ReflectionMetric::score("short answer", "longer improved answer with more detail");
    assert_eq!(score, 1.0);
}

#[test]
fn reflection_no_change_scores_zero() {
    let score = ReflectionMetric::score("same output", "same output");
    assert_eq!(score, 0.0);
}

#[test]
fn reflection_changed_but_shorter_scores_half() {
    let score = ReflectionMetric::score("longer original answer", "short");
    assert_eq!(score, 0.5);
}
