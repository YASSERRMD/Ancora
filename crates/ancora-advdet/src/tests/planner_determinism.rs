use ancora_orchestrate::{fan_out, AgentTask};
use serde_json::json;

fn make_tasks(tick: u64) -> Vec<AgentTask> {
    let inputs = vec![json!("plan-A"), json!("plan-B"), json!("plan-C")];
    fan_out("orch-1", "planner", inputs, "root")
}

#[test]
fn planner_replay_task_count() {
    let run1 = make_tasks(1);
    let run2 = make_tasks(1);
    assert_eq!(run1.len(), run2.len());
}

#[test]
fn planner_replay_task_ids_stable() {
    let run1 = make_tasks(1);
    let run2 = make_tasks(1);
    let ids1: Vec<&str> = run1.iter().map(|t| t.task_id.as_str()).collect();
    let ids2: Vec<&str> = run2.iter().map(|t| t.task_id.as_str()).collect();
    assert_eq!(ids1, ids2);
}

#[test]
fn planner_replay_inputs_stable() {
    let run1 = make_tasks(1);
    let run2 = make_tasks(1);
    let ins1: Vec<&serde_json::Value> = run1.iter().map(|t| &t.input).collect();
    let ins2: Vec<&serde_json::Value> = run2.iter().map(|t| &t.input).collect();
    assert_eq!(ins1, ins2);
}
