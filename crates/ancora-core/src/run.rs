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

impl RunStatus {
    /// Returns true if no further transitions are legal from this state.
    pub fn is_terminal(self) -> bool {
        matches!(self, RunStatus::Completed | RunStatus::Cancelled | RunStatus::Failed)
    }

    /// Attempt to transition to `next`. Returns an error if the transition is
    /// not allowed by the state machine.
    ///
    /// Legal transitions:
    ///   Pending   -> Running
    ///   Running   -> Completed | Cancelled | Failed
    pub fn transition(self, next: RunStatus) -> Result<RunStatus, AncoraError> {
        let allowed = match self {
            RunStatus::Pending => matches!(next, RunStatus::Running),
            RunStatus::Running => {
                matches!(next, RunStatus::Completed | RunStatus::Cancelled | RunStatus::Failed)
            }
            RunStatus::Completed | RunStatus::Cancelled | RunStatus::Failed => false,
        };
        if allowed {
            Ok(next)
        } else {
            Err(AncoraError::InvalidState(format!(
                "cannot transition from {self:?} to {next:?}"
            )))
        }
    }
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

    /// Transition the run to `next` status, updating `self.status` on success.
    pub fn transition(&mut self, next: RunStatus) -> Result<(), AncoraError> {
        self.status = self.status.transition(next)?;
        Ok(())
    }
}
