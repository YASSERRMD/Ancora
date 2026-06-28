use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("unknown tool: {name}")]
    UnknownTool { name: String },
    #[error("tool call timed out: {tool} after {ms}ms")]
    Timeout { tool: String, ms: u64 },
    #[error("tool execution failed: {tool}: {reason}")]
    ExecutionFailed { tool: String, reason: String },
    #[error("invalid arguments for tool {tool}: {reason}")]
    InvalidArguments { tool: String, reason: String },
    #[error("parallel call limit exceeded: limit is {limit}")]
    ParallelLimitExceeded { limit: usize },
}
