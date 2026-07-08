/// End-to-end policy enforcement tests (offline).
///
/// Validates that policy-driven errors (residency, cost cap, step limit)
/// are classified as terminal, stop retries immediately, and produce the
/// correct journal representation when a run is blocked by policy.
use ancora_core::{
    error::AncoraError,
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    retry::{classify, run_with_retry, ErrorClass, RetryOutcome, RetryPolicy},
    run::RunStatus,
};
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, RunCancelledEvent, RunStartedEvent,
};

fn ev(seq: u64, run_id: &str, event: Event) -> JournalEvent {
    JournalEvent {
        event_id: format!("{}-{}", run_id, seq),
        run_id: run_id.to_owned(),
        seq,
        recorded_at_ns: (seq * 1_000_000) as i64,
        event: Some(event),
    }
}

fn policy_blocked_journal(run_id: &str, reason: &str) -> Vec<JournalEvent> {
    vec![
        ev(
            0,
            run_id,
            Event::RunStarted(RunStartedEvent {
                run_id: run_id.to_owned(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            }),
        ),
        ev(
            1,
            run_id,
            Event::RunCancelled(RunCancelledEvent {
                reason: reason.to_owned(),
            }),
        ),
    ]
}

fn no_retry_policy() -> RetryPolicy {
    RetryPolicy {
        max_attempts: 5,
        initial_backoff_ms: 0,
        max_backoff_ms: 0,
        jitter: 0.0,
    }
}

#[test]
fn residency_policy_violation_is_terminal_error_class() {
    let err = AncoraError::PolicyResidency("eu-only data must stay in EU".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn max_steps_exceeded_is_terminal_error_class() {
    let err = AncoraError::MaxSteps { max_steps: 10 };
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn graph_invalid_is_terminal_error_class() {
    let err = AncoraError::GraphInvalid("cycle detected".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn cancelled_is_terminal_error_class() {
    let err = AncoraError::Cancelled("user cancelled".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn policy_violation_stops_retry_immediately() {
    let policy = no_retry_policy();
    let mut calls = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            Err::<(), _>(AncoraError::PolicyResidency("eu-only".into()))
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Terminal { attempt: 1, .. }));
    assert_eq!(calls, 1, "policy violation must stop after first call");
}

#[test]
fn max_steps_stops_retry_immediately() {
    let policy = no_retry_policy();
    let mut calls = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            Err::<(), _>(AncoraError::MaxSteps { max_steps: 10 })
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Terminal { .. }));
    assert_eq!(calls, 1);
}

#[test]
fn policy_blocked_journal_replays_without_completed_status() {
    let run_id = "policy-blocked";
    let events = policy_blocked_journal(run_id, "eu-residency violation");
    let state = replay_events(run_id, &events).unwrap();
    assert_ne!(
        state.run.status,
        RunStatus::Completed,
        "blocked run must not show Completed"
    );
}

#[test]
fn policy_blocked_journal_stores_without_error() {
    let store = MemoryStore::new();
    let run_id = "policy-store";

    for ev in policy_blocked_journal(run_id, "residency") {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 2);
}

#[test]
fn nondeterminism_is_terminal_error_class() {
    let err = AncoraError::Nondeterminism {
        seq: 3,
        expected: "a".into(),
        got: "b".into(),
    };
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn policy_error_message_preserved_in_cancelled_event() {
    let reason = "data must not leave eu-west";
    let events = policy_blocked_journal("policy-msg", reason);

    if let Some(Event::RunCancelled(c)) = &events[1].event {
        assert_eq!(
            c.reason, reason,
            "cancelled event must preserve policy reason"
        );
    } else {
        panic!("second event must be RunCancelled");
    }
}

#[test]
fn retryable_then_terminal_exhausts_on_terminal() {
    let policy = no_retry_policy();
    let mut calls = 0u32;

    let outcome = run_with_retry(
        &policy,
        |_| {
            calls += 1;
            if calls == 1 {
                Err::<(), _>(AncoraError::ModelUnreachable("transient".into()))
            } else {
                Err::<(), _>(AncoraError::Cancelled("user cancelled".into()))
            }
        },
        |_| {},
    );

    assert!(matches!(outcome, RetryOutcome::Terminal { attempt: 2, .. }));
}
