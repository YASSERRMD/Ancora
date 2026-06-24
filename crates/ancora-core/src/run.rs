use crate::error::AncoraError;

/// Lifecycle state of a single run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    /// Created but not yet executing.
    Pending,
    /// Executing the agent loop.
    Running,
    /// Finished successfully.
    Completed,
    /// Cancelled by an external signal.
    Cancelled,
    /// Terminated with an error.
    Failed,
}

/// A running or terminated instance of an agent.
#[derive(Debug, Clone)]
pub struct Run {
    /// Stable, globally unique identifier for this run.
    pub id: String,
    /// Current lifecycle state.
    pub status: RunStatus,
    /// Sequence number of the last journal event appended for this run.
    pub seq: u64,
}

impl Run {
    /// Create a new run in the `Pending` state.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: RunStatus::Pending,
            seq: 0,
        }
    }
}
