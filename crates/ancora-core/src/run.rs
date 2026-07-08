use uuid::Uuid;

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
        matches!(
            self,
            RunStatus::Completed | RunStatus::Cancelled | RunStatus::Failed
        )
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
                matches!(
                    next,
                    RunStatus::Completed | RunStatus::Cancelled | RunStatus::Failed
                )
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
    /// Create a new run in the `Pending` state with a caller-supplied id.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: RunStatus::Pending,
            seq: 0,
        }
    }

    /// Create a new run whose id is a freshly generated UUID v4.
    ///
    /// Use this as the sole entry point for ID generation so callers are never
    /// tempted to construct IDs inline. The ID is stable once assigned; it is
    /// not re-generated on journal replay.
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4().to_string())
    }

    /// Transition the run to `next` status, updating `self.status` on success.
    pub fn transition(&mut self, next: RunStatus) -> Result<(), AncoraError> {
        self.status = self.status.transition(next)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_transitions_follow_legal_state_machine() {
        let mut run = Run::new("test-run-1");
        assert_eq!(run.status, RunStatus::Pending);

        run.transition(RunStatus::Running).unwrap();
        assert_eq!(run.status, RunStatus::Running);

        run.transition(RunStatus::Completed).unwrap();
        assert_eq!(run.status, RunStatus::Completed);
        assert!(run.status.is_terminal());
    }

    #[test]
    fn run_can_be_cancelled_from_running() {
        let mut run = Run::new("test-run-2");
        run.transition(RunStatus::Running).unwrap();
        run.transition(RunStatus::Cancelled).unwrap();
        assert!(run.status.is_terminal());
    }

    #[test]
    fn run_can_fail_from_running() {
        let mut run = Run::new("test-run-3");
        run.transition(RunStatus::Running).unwrap();
        run.transition(RunStatus::Failed).unwrap();
        assert!(run.status.is_terminal());
    }

    #[test]
    fn illegal_transitions_are_rejected() {
        let cases: Vec<(RunStatus, RunStatus)> = vec![
            (RunStatus::Pending, RunStatus::Completed),
            (RunStatus::Pending, RunStatus::Cancelled),
            (RunStatus::Pending, RunStatus::Failed),
            (RunStatus::Pending, RunStatus::Pending),
            (RunStatus::Running, RunStatus::Pending),
            (RunStatus::Running, RunStatus::Running),
            (RunStatus::Completed, RunStatus::Running),
            (RunStatus::Completed, RunStatus::Completed),
            (RunStatus::Cancelled, RunStatus::Running),
            (RunStatus::Failed, RunStatus::Running),
        ];
        for (from, to) in cases {
            let err = from.transition(to).unwrap_err();
            assert!(
                matches!(err, AncoraError::InvalidState(_)),
                "expected InvalidState for {from:?} -> {to:?}, got {err:?}"
            );
        }
    }

    #[test]
    fn generate_produces_unique_ids() {
        let a = Run::generate();
        let b = Run::generate();
        assert_ne!(a.id, b.id, "IDs must be unique");
        assert!(!a.id.is_empty());
    }

    #[test]
    fn terminal_states_are_correct() {
        assert!(!RunStatus::Pending.is_terminal());
        assert!(!RunStatus::Running.is_terminal());
        assert!(RunStatus::Completed.is_terminal());
        assert!(RunStatus::Cancelled.is_terminal());
        assert!(RunStatus::Failed.is_terminal());
    }
}
