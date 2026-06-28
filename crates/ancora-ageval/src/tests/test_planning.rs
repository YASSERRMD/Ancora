use crate::planning::PlanningMetric;

#[test]
fn planning_metric_on_fixture() {
    let expected = vec!["step_a".into(), "step_b".into(), "step_c".into()];
    let actual = vec!["step_a".into(), "step_b".into(), "step_c".into()];
    assert!((PlanningMetric::score(&expected, &actual) - 1.0).abs() < 1e-10);
}

#[test]
fn planning_partial_match_scores_fraction() {
    let expected = vec!["step_a".into(), "step_b".into(), "step_c".into()];
    let actual = vec!["step_a".into(), "step_b".into()];
    let score = PlanningMetric::score(&expected, &actual);
    assert!((score - 2.0 / 3.0).abs() < 1e-10);
}

#[test]
fn planning_empty_expected_is_perfect() {
    let score = PlanningMetric::score(&[], &["step_a".to_string()]);
    assert_eq!(score, 1.0);
}

#[test]
fn planning_no_match_scores_zero() {
    let expected = vec!["step_a".into()];
    let actual = vec!["step_b".into()];
    assert_eq!(PlanningMetric::score(&expected, &actual), 0.0);
}
