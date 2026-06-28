use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestrateError {
    #[error("task cycle detected in task graph")]
    CycleDetected,
    #[error("task not found: {task_id}")]
    TaskNotFound { task_id: String },
    #[error("agent not registered: {agent_id}")]
    AgentNotFound { agent_id: String },
    #[error("max subagent spawn depth exceeded: {depth}")]
    MaxDepthExceeded { depth: usize },
    #[error("task failed: {task_id}: {reason}")]
    TaskFailed { task_id: String, reason: String },
}
