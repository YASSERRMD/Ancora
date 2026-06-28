use crate::agent_spec::AgentTask;
use crate::spawn::SpawnTracker;
use serde_json::json;

#[test]
fn spawn_records_are_tracked() {
    let mut tracker = SpawnTracker::new();
    let task = AgentTask::new("t1", "subagent-1", json!({}));
    tracker.spawn("orchestrator", task, 0);
    assert_eq!(tracker.total_spawned(), 1);
}

#[test]
fn by_parent_filters_correctly() {
    let mut tracker = SpawnTracker::new();
    tracker.spawn("orch", AgentTask::new("t1", "a", json!({})), 0);
    tracker.spawn("orch", AgentTask::new("t2", "b", json!({})), 0);
    tracker.spawn("other", AgentTask::new("t3", "c", json!({})), 0);
    assert_eq!(tracker.by_parent("orch").len(), 2);
    assert_eq!(tracker.by_parent("other").len(), 1);
}
