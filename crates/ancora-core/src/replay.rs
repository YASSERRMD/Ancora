use ancora_proto::ancora::{journal_event::Event, JournalEvent};

use crate::error::AncoraError;
use crate::run::{Run, RunStatus};

/// Derived state produced by folding a journal into a Run.
#[derive(Debug, Clone)]
pub struct ReplayState {
    /// The run reconstructed from journal events.
    pub run: Run,
    /// Ordered list of activity keys seen during replay (in journal order).
    pub activity_keys: Vec<String>,
}

impl ReplayState {
    fn new(run_id: &str) -> Self {
        Self {
            run: Run::new(run_id),
            activity_keys: Vec::new(),
        }
    }
}

/// Fold all events for `run_id` from `events` into a `ReplayState`.
///
/// Events are applied in seq order. Unknown event kinds are silently
/// skipped so the engine is forward-compatible with new event types.
pub fn replay_events(run_id: &str, events: &[JournalEvent]) -> Result<ReplayState, AncoraError> {
    let mut state = ReplayState::new(run_id);

    for event in events {
        apply_event(&mut state, event)?;
    }

    Ok(state)
}

fn apply_event(state: &mut ReplayState, event: &JournalEvent) -> Result<(), AncoraError> {
    match event.event.as_ref() {
        Some(Event::RunStarted(_)) => {
            if state.run.status == RunStatus::Pending {
                state.run.transition(RunStatus::Running)?;
            }
        }
        Some(Event::RunCompleted(_)) => {
            if state.run.status == RunStatus::Running {
                state.run.transition(RunStatus::Completed)?;
            }
        }
        Some(Event::RunCancelled(_)) => {
            if state.run.status == RunStatus::Running {
                state.run.transition(RunStatus::Cancelled)?;
            }
        }
        Some(Event::ActivityRecorded(a)) => {
            state.run.seq = event.seq;
            state.activity_keys.push(a.activity_key.clone());
        }
        Some(Event::Error(_)) => {
            if state.run.status == RunStatus::Running {
                state.run.transition(RunStatus::Failed)?;
            }
        }
        _ => {}
    }
    Ok(())
}
