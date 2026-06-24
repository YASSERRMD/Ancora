/// Errors returned by tool operations.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("schema validation failed: {0}")]
    ValidationFailed(String),
    #[error("tool not found: {0}")]
    NotFound(String),
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
}
