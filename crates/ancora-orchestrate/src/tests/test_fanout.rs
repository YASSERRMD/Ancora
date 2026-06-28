use crate::fanout::fan_out;
use serde_json::json;

#[test]
fn fan_out_produces_correct_count() {
    let inputs = vec![json!({"x": 1}), json!({"x": 2}), json!({"x": 3})];
    let tasks = fan_out("orch", "subagent", inputs, "parent-1");
    assert_eq!(tasks.len(), 3);
}

#[test]
fn fan_out_sets_parent_task_id() {
    let inputs = vec![json!({"v": 1}), json!({"v": 2})];
    let tasks = fan_out("orch", "subagent", inputs, "p-99");
    for t in &tasks {
        assert_eq!(t.parent_task_id.as_deref(), Some("p-99"));
    }
}

#[test]
fn fan_out_empty_inputs() {
    let tasks = fan_out("orch", "sub", vec![], "root");
    assert!(tasks.is_empty());
}

#[test]
fn fan_out_task_ids_unique() {
    let inputs = vec![json!(1), json!(2), json!(3)];
    let tasks = fan_out("orch", "sub", inputs, "base");
    let ids: Vec<&str> = tasks.iter().map(|t| t.task_id.as_str()).collect();
    let mut deduped = ids.clone();
    deduped.sort();
    deduped.dedup();
    assert_eq!(ids.len(), deduped.len());
}
