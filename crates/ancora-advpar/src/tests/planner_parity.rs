use ancora_ageval::PlanningMetric;
use ancora_orchestrate::fan_out;
use serde_json::json;

const EPS: f64 = 1e-9;

#[test]
fn planner_parity_three_of_four() {
    let expected: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
    let actual: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let score = PlanningMetric::score(&expected, &actual);
    assert!((score - 0.75).abs() < EPS, "got {score}");
}

#[test]
fn planner_parity_perfect() {
    let steps: Vec<String> = vec!["x".into(), "y".into()];
    assert!((PlanningMetric::score(&steps, &steps) - 1.0).abs() < EPS);
}

#[test]
fn planner_parity_empty_expected() {
    let score = PlanningMetric::score(&[], &["x".to_string()]);
    assert!((score - 1.0).abs() < EPS);
}

#[test]
fn planner_parity_fan_out_count() {
    let tasks = fan_out(
        "o",
        "a",
        vec![json!("t1"), json!("t2"), json!("t3")],
        "root",
    );
    assert_eq!(tasks.len(), 3);
}

#[test]
fn planner_parity_fan_out_ids_deterministic() {
    let t1 = fan_out("o", "a", vec![json!("x")], "root");
    let t2 = fan_out("o", "a", vec![json!("x")], "root");
    assert_eq!(t1[0].task_id, t2[0].task_id);
}
