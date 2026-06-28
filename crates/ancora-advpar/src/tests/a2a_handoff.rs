// Polyglot A2A handoff: verifies that state transferred between agents
// (simulated as shared journals and task IDs) is stable across language boundary.
use ancora_coord::CoordJournal;
use ancora_orchestrate::fan_out;
use serde_json::json;

fn rust_agent_handoff() -> (Vec<String>, Vec<(String, String)>) {
    let tasks = fan_out("orchestrator", "agent-go", vec![json!("task-A")], "root");
    let task_ids: Vec<String> = tasks.iter().map(|t| t.task_id.clone()).collect();

    let mut journal = CoordJournal::default();
    journal.record(1, "handoff", &format!("rust -> go: {}", task_ids[0]));
    let events: Vec<(String, String)> = journal
        .events()
        .iter()
        .map(|e| (e.kind.clone(), e.description.clone()))
        .collect();

    (task_ids, events)
}

#[test]
fn a2a_handoff_task_id_stable() {
    let (ids1, _) = rust_agent_handoff();
    let (ids2, _) = rust_agent_handoff();
    assert_eq!(ids1, ids2);
}

#[test]
fn a2a_handoff_journal_events_stable() {
    let (_, events1) = rust_agent_handoff();
    let (_, events2) = rust_agent_handoff();
    assert_eq!(events1, events2);
}

#[test]
fn a2a_handoff_description_contains_task_id() {
    let (ids, events) = rust_agent_handoff();
    assert!(events[0].1.contains(&ids[0]));
}

#[test]
fn a2a_handoff_agent_id_in_task() {
    let tasks = fan_out("orchestrator", "agent-python", vec![json!("work")], "root");
    assert_eq!(tasks[0].agent_id, "agent-python");
}
