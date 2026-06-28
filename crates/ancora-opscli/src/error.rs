use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpsCLIError {
    #[error("run not found: {0}")]
    RunNotFound(String),
    #[error("worker not found: {0}")]
    WorkerNotFound(String),
    #[error("tenant not found: {0}")]
    TenantNotFound(String),
    #[error("operation not allowed in current state: {0}")]
    InvalidState(String),
    #[error("serialization error: {0}")]
    SerdeError(String),
}
