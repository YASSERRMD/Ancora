use crate::agent_spec::{AgentRole, AgentSpec, AgentTask};
use serde_json::json;

#[test]
fn agent_spec_defaults() {
    let spec = AgentSpec::new("a1", AgentRole::Subagent, "You are a helper");
    assert_eq!(spec.max_turns, 10);
    assert_eq!(spec.model, "claude-sonnet-4-6");
}

#[test]
fn agent_task_with_parent() {
    let t = AgentTask::new("t1", "a1", json!({})).with_parent("t0");
    assert_eq!(t.parent_task_id, Some("t0".to_string()));
}

#[test]
fn agent_spec_with_tools() {
    let spec =
        AgentSpec::new("a1", AgentRole::Critic, "critique").with_tools(vec!["search", "read"]);
    assert_eq!(spec.tools.len(), 2);
}
