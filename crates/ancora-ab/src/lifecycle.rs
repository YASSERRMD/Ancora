/// Manages the lifecycle of an experiment: pending, running, stopped, concluded.

use std::time::{SystemTime, UNIX_EPOCH};

/// The current state of an experiment.
#[derive(Debug, Clone, PartialEq)]
pub enum ExperimentState {
    /// Created but not yet started.
    Pending,
    /// Actively running and collecting data.
    Running { started_at: u64 },
    /// Manually stopped before reaching a conclusion.
    Stopped { started_at: u64, stopped_at: u64 },
    /// Concluded with a winning variant (or inconclusive).
    Concluded {
        started_at: u64,
        concluded_at: u64,
        winner: Option<String>,
    },
}

/// Error conditions for lifecycle transitions.
#[derive(Debug, PartialEq)]
pub enum LifecycleError {
    AlreadyRunning,
    NotRunning,
    AlreadyConcluded,
    AlreadyStopped,
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleError::AlreadyRunning => write!(f, "experiment is already running"),
            LifecycleError::NotRunning => write!(f, "experiment is not running"),
            LifecycleError::AlreadyConcluded => write!(f, "experiment has already concluded"),
            LifecycleError::AlreadyStopped => write!(f, "experiment has already been stopped"),
        }
    }
}

/// Tracks lifecycle state for a single experiment.
#[derive(Debug, Clone)]
pub struct LifecycleManager {
    pub experiment_id: String,
    pub state: ExperimentState,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl LifecycleManager {
    pub fn new(experiment_id: impl Into<String>) -> Self {
        LifecycleManager {
            experiment_id: experiment_id.into(),
            state: ExperimentState::Pending,
        }
    }

    /// Transition to Running.
    pub fn start(&mut self) -> Result<(), LifecycleError> {
        match &self.state {
            ExperimentState::Pending => {
                self.state = ExperimentState::Running {
                    started_at: now_secs(),
                };
                Ok(())
            }
            ExperimentState::Running { .. } => Err(LifecycleError::AlreadyRunning),
            ExperimentState::Concluded { .. } => Err(LifecycleError::AlreadyConcluded),
            ExperimentState::Stopped { .. } => Err(LifecycleError::AlreadyStopped),
        }
    }

    /// Transition to Stopped (manual halt, no winner declared).
    pub fn stop(&mut self) -> Result<(), LifecycleError> {
        match &self.state {
            ExperimentState::Running { started_at } => {
                self.state = ExperimentState::Stopped {
                    started_at: *started_at,
                    stopped_at: now_secs(),
                };
                Ok(())
            }
            ExperimentState::Pending | ExperimentState::Stopped { .. } => {
                Err(LifecycleError::NotRunning)
            }
            ExperimentState::Concluded { .. } => Err(LifecycleError::AlreadyConcluded),
        }
    }

    /// Transition to Concluded with an optional winner.
    pub fn conclude(&mut self, winner: Option<String>) -> Result<(), LifecycleError> {
        match &self.state {
            ExperimentState::Running { started_at } => {
                self.state = ExperimentState::Concluded {
                    started_at: *started_at,
                    concluded_at: now_secs(),
                    winner,
                };
                Ok(())
            }
            ExperimentState::Pending | ExperimentState::Stopped { .. } => {
                Err(LifecycleError::NotRunning)
            }
            ExperimentState::Concluded { .. } => Err(LifecycleError::AlreadyConcluded),
        }
    }

    /// Returns true when the experiment is actively collecting data.
    pub fn is_running(&self) -> bool {
        matches!(self.state, ExperimentState::Running { .. })
    }

    /// Returns the winning variant name, if the experiment concluded with one.
    pub fn winner(&self) -> Option<&str> {
        match &self.state {
            ExperimentState::Concluded {
                winner: Some(w), ..
            } => Some(w.as_str()),
            _ => None,
        }
    }
}
