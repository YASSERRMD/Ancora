use ancora_ageval::ReflectionMetric;
use ancora_orchestrate::fan_out;

#[test]
fn planner_plus_reflection_pipeline() {
    let inputs = vec![serde_json::json!("step-a"), serde_json::json!("step-b")];
    let tasks = fan_out("orch-1", "planner", inputs, "root");
    assert_eq!(tasks.len(), 2);

    // Simulate reflection: each task input is refined (longer = improvement)
    for task in &tasks {
        let before = task.input.as_str().unwrap_or("step");
        let after = format!("{} (with reflection)", before);
        let score = ReflectionMetric::score(before, &after);
        assert_eq!(score, 1.0);
    }
}

#[test]
fn reflection_no_change_stays_at_zero() {
    let score = ReflectionMetric::score("unchanged output", "unchanged output");
    assert_eq!(score, 0.0);
}
