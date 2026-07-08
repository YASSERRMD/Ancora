use std::thread;

use ancora_core::{
    cancel::cancellation_pair,
    error::AncoraError,
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
    retry::{classify, ErrorClass},
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

fn cancelled_journal(run_id: &str, reason: &str) -> Vec<JournalEvent> {
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

#[test]
fn cancellation_pair_token_starts_not_cancelled() {
    let (token, _handle) = cancellation_pair();
    assert!(!token.is_cancelled());
}

#[test]
fn cancellation_pair_cancel_sets_token() {
    let (token, handle) = cancellation_pair();
    handle.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn cancellation_error_is_terminal_in_retry() {
    let err = AncoraError::Cancelled("user requested stop".into());
    assert_eq!(classify(&err), ErrorClass::Terminal);
}

#[test]
fn cancelled_journal_does_not_replay_to_completed() {
    let run_id = "cancel-e2e-1";
    let events = cancelled_journal(run_id, "user cancelled");
    let state = replay_events(run_id, &events).unwrap();
    assert_ne!(state.run.status, RunStatus::Completed);
}

#[test]
fn cancelled_journal_has_two_events() {
    let events = cancelled_journal("cancel-count", "reason");
    assert_eq!(events.len(), 2);
}

#[test]
fn cancelled_event_preserves_reason() {
    let reason = "admin shutdown";
    let events = cancelled_journal("cancel-reason", reason);
    if let Some(Event::RunCancelled(c)) = &events[1].event {
        assert_eq!(c.reason, reason);
    } else {
        panic!("second event must be RunCancelled");
    }
}

#[test]
fn multiple_cancellations_on_same_handle_are_idempotent() {
    let (token, handle) = cancellation_pair();
    handle.cancel();
    handle.cancel();
    handle.cancel();
    assert!(
        token.is_cancelled(),
        "token must be cancelled after multiple cancel calls"
    );
}

#[test]
fn token_clone_shares_cancellation_state() {
    let (token, handle) = cancellation_pair();
    let token2 = token.clone();
    assert!(!token2.is_cancelled());
    handle.cancel();
    assert!(
        token2.is_cancelled(),
        "cloned token must see the cancellation"
    );
}

#[test]
fn cancellation_propagates_across_threads() {
    let (token, handle) = cancellation_pair();
    let token_clone = token.clone();

    let t = thread::spawn(move || {
        while !token_clone.is_cancelled() {
            thread::yield_now();
        }
        true
    });

    handle.cancel();
    assert!(t.join().unwrap(), "thread must observe cancellation");
}

#[test]
fn cancelled_run_journal_stores_without_error() {
    let store = MemoryStore::new();
    let run_id = "cancel-store";

    for ev in cancelled_journal(run_id, "user quit") {
        store.append(run_id, ev).unwrap();
    }

    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 2);
}

#[test]
fn ten_concurrent_cancel_checks_are_all_consistent() {
    let (token, handle) = cancellation_pair();
    handle.cancel();

    let results: Vec<bool> = (0..10).map(|_| token.is_cancelled()).collect();
    assert!(
        results.iter().all(|&r| r),
        "all checks must agree the token is cancelled"
    );
}
