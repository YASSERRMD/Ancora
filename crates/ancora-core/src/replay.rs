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
        Some(Event::Error(_)) if state.run.status == RunStatus::Running => {
            state.run.transition(RunStatus::Failed)?;
        }
        _ => {}
    }
    Ok(())
}

/// Check whether the sequence of activity keys produced by the current code
/// path (`observed`) matches the sequence recorded in the journal (`expected`).
///
/// Returns `AncoraError::Nondeterminism` with position details if any key
/// diverges. An extra observed key beyond the journal end is also a divergence.
pub fn detect_divergence(expected: &[String], observed: &[String]) -> Result<(), AncoraError> {
    for (seq, (exp, obs)) in expected.iter().zip(observed.iter()).enumerate() {
        if exp != obs {
            return Err(AncoraError::Nondeterminism {
                seq: seq as u64,
                expected: exp.clone(),
                got: obs.clone(),
            });
        }
    }
    if observed.len() > expected.len() {
        return Err(AncoraError::Nondeterminism {
            seq: expected.len() as u64,
            expected: "<end-of-journal>".to_string(),
            got: observed[expected.len()].clone(),
        });
    }
    if observed.len() < expected.len() {
        return Err(AncoraError::Nondeterminism {
            seq: observed.len() as u64,
            expected: expected[observed.len()].clone(),
            got: "<end-of-observed>".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use ancora_proto::ancora::{
        journal_event::Event, ActivityRecordedEvent, JournalEvent, RunStartedEvent,
    };

    use super::*;

    fn run_started(run_id: &str, seq: u64) -> JournalEvent {
        JournalEvent {
            event_id: format!("evt-{seq}"),
            run_id: run_id.to_string(),
            seq,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: run_id.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".to_string(),
            })),
        }
    }

    fn activity_recorded(run_id: &str, seq: u64, key: &str) -> JournalEvent {
        JournalEvent {
            event_id: format!("evt-{seq}"),
            run_id: run_id.to_string(),
            seq,
            recorded_at_ns: 0,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: key.to_string(),
                activity_kind: "model_call".to_string(),
                input_json: "{}".to_string(),
                result_json: r#""ok""#.to_string(),
                replayed: false,
            })),
        }
    }

    #[test]
    fn replay_reproduces_identical_state() {
        let events = vec![
            run_started("run-r1", 0),
            activity_recorded("run-r1", 1, "step-a"),
            activity_recorded("run-r1", 2, "step-b"),
        ];
        let state = replay_events("run-r1", &events).unwrap();

        assert_eq!(state.run.status, RunStatus::Running);
        assert_eq!(state.run.seq, 2);
        assert_eq!(state.activity_keys, vec!["step-a", "step-b"]);
    }

    #[test]
    fn replay_empty_journal_gives_pending_run() {
        let state = replay_events("run-r2", &[]).unwrap();
        assert_eq!(state.run.status, RunStatus::Pending);
        assert!(state.activity_keys.is_empty());
    }

    #[test]
    fn injected_code_change_triggers_divergence_error() {
        let expected = vec!["step-a".to_string(), "step-b".to_string()];
        let observed = vec!["step-a".to_string(), "step-X".to_string()];

        let err = detect_divergence(&expected, &observed).unwrap_err();
        assert!(
            matches!(err, AncoraError::Nondeterminism { seq: 1, .. }),
            "expected Nondeterminism at seq 1, got {err:?}"
        );
    }

    #[test]
    fn extra_observed_step_triggers_divergence_error() {
        let expected = vec!["step-a".to_string()];
        let observed = vec!["step-a".to_string(), "step-extra".to_string()];

        let err = detect_divergence(&expected, &observed).unwrap_err();
        assert!(
            matches!(err, AncoraError::Nondeterminism { seq: 1, .. }),
            "expected Nondeterminism at seq 1, got {err:?}"
        );
    }

    #[test]
    fn identical_keys_pass_divergence_check() {
        let keys = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        detect_divergence(&keys, &keys).unwrap();
    }
}
