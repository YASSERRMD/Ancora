use crate::agent_spec::AgentTask;
use serde_json::Value;

/// Creates parallel subagent tasks from a list of inputs.
pub fn fan_out(
    _orchestrator_id: &str,
    agent_id: &str,
    inputs: Vec<Value>,
    parent_task_id: &str,
) -> Vec<AgentTask> {
    inputs
        .into_iter()
        .enumerate()
        .map(|(i, input)| {
            AgentTask::new(&format!("{parent_task_id}-fanout-{i}"), agent_id, input)
                .with_parent(parent_task_id)
        })
        .collect()
}
