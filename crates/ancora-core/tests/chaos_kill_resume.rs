/// Kill-mid-run and resume-to-completion chaos tests.
///
/// Each test simulates a process kill at a specific point by leaving the
/// journal in a partially-written state, then resumes and verifies the run
/// completes to the correct final state with no duplicate side effects.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use ancora_core::{
    activity::Activity,
    error::AncoraError,
    journal::{JournalStore, MemoryStore},
    replay::replay_events,
};
use ancora_proto::ancora::{
    journal_event::Event, JournalEvent, RunStartedEvent, RunCompletedEvent,
};

struct SideEffectActivity {
    key: String,
    counter: Arc<AtomicUsize>,
}

impl Activity for SideEffectActivity {
    fn key(&self) -> String { self.key.clone() }
    fn execute(&self) -> Result<String, AncoraError> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        Ok(format!(r#"{{"done":true}}"#))
    }
}

fn append_started(store: &MemoryStore, run_id: &str) {
    store.append(run_id, JournalEvent {
        event_id: format!("{}-started", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunStarted(RunStartedEvent {
            run_id: run_id.to_owned(),
            spec_bytes: vec![],
            spec_type: "AgentSpec".into(),
        })),
    }).unwrap();
}

fn append_completed(store: &MemoryStore, run_id: &str) {
    store.append(run_id, JournalEvent {
        event_id: format!("{}-completed", run_id),
        run_id: run_id.to_owned(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::RunCompleted(RunCompletedEvent {
            output_json: String::new(),
        })),
    }).unwrap();
}

fn record_activity(store: &MemoryStore, run_id: &str, counter: &Arc<AtomicUsize>, key: &str) {
    use ancora_core::idempotency::{write_once, WriteActivity};
    let act = SideEffectActivity { key: key.into(), counter: Arc::clone(counter) };
    let wa = WriteActivity::new(&act).unwrap();
    write_once(run_id, wa, store).unwrap();
}

#[test]
fn kill_before_any_activity_resumes_to_completion() {
    let store = MemoryStore::new();
    let run_id = "run-kill-before";
    let counter = Arc::new(AtomicUsize::new(0));

    // Crash: only started event is journaled.
    append_started(&store, run_id);

    // Resume: activity executes, then run completes.
    record_activity(&store, run_id, &counter, "act-1");
    append_completed(&store, run_id);

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(format!("{:?}", state.run.status), "Completed");
    assert_eq!(counter.load(Ordering::SeqCst), 1, "activity must execute exactly once");
}

#[test]
fn kill_after_activity_resumes_without_re_executing() {
    let store = MemoryStore::new();
    let run_id = "run-kill-after-act";
    let counter = Arc::new(AtomicUsize::new(0));

    // First run: activity executes and is journaled, then crash.
    append_started(&store, run_id);
    record_activity(&store, run_id, &counter, "act-1");
    // Crash here -- completed event is NOT written.

    assert_eq!(counter.load(Ordering::SeqCst), 1, "first run must execute once");

    // Resume: activity should replay from journal, not re-execute.
    record_activity(&store, run_id, &counter, "act-1");
    append_completed(&store, run_id);

    assert_eq!(counter.load(Ordering::SeqCst), 1, "resume must not re-execute journaled activity");

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(format!("{:?}", state.run.status), "Completed");
}

#[test]
fn multiple_crashes_accumulate_no_duplicate_side_effects() {
    let store = MemoryStore::new();
    let run_id = "run-multi-crash";
    let steps = 5;
    let counters: Vec<Arc<AtomicUsize>> = (0..steps)
        .map(|_| Arc::new(AtomicUsize::new(0)))
        .collect();

    // Simulated run 1: execute steps 0..2 then crash.
    append_started(&store, run_id);
    for i in 0..2 {
        record_activity(&store, run_id, &counters[i], &format!("act-{}", i));
    }

    // Simulated run 2: resume and execute steps 2..4 then crash.
    for i in 0..4 {
        record_activity(&store, run_id, &counters[i], &format!("act-{}", i));
    }

    // Simulated run 3: resume and complete all steps.
    for i in 0..steps {
        record_activity(&store, run_id, &counters[i], &format!("act-{}", i));
    }
    append_completed(&store, run_id);

    // Every activity must have been executed exactly once.
    for (i, c) in counters.iter().enumerate() {
        assert_eq!(
            c.load(Ordering::SeqCst), 1,
            "activity {} must execute exactly once across all crash/resume cycles", i
        );
    }

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    assert_eq!(format!("{:?}", state.run.status), "Completed");
}

#[test]
fn replay_after_crash_gives_same_activity_key_list() {
    let store = MemoryStore::new();
    let run_id = "run-replay-keys";
    let keys = ["k1", "k2", "k3"];
    let counter = Arc::new(AtomicUsize::new(0));

    append_started(&store, run_id);
    for key in &keys {
        record_activity(&store, run_id, &counter, key);
    }
    append_completed(&store, run_id);

    let events = store.read(run_id).unwrap();
    let state = replay_events(run_id, &events).unwrap();
    let found: Vec<&str> = state.activity_keys.iter().map(|s| s.as_str()).collect();
    assert_eq!(found, keys.as_slice());
}
