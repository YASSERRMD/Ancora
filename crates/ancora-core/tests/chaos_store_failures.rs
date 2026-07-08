/// Store-failure injection chaos tests.
///
/// A `FaultyStore` wraps `MemoryStore` and injects `AncoraError::Storage`
/// errors at configurable points.  Tests verify that the engine handles
/// write failures gracefully (returns error, does not corrupt existing state)
/// and that operations succeeding after a failure are consistent.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use ancora_core::{
    error::AncoraError,
    journal::{JournalStore, MemoryStore},
};
use ancora_proto::ancora::JournalEvent;

/// Wraps MemoryStore and fails the next `fail_count` append operations.
struct FaultyStore {
    inner: MemoryStore,
    remaining_failures: Arc<Mutex<usize>>,
}

impl FaultyStore {
    fn new(fail_count: usize) -> Self {
        Self {
            inner: MemoryStore::new(),
            remaining_failures: Arc::new(Mutex::new(fail_count)),
        }
    }
}

impl JournalStore for FaultyStore {
    fn append(&self, run_id: &str, event: JournalEvent) -> Result<u64, AncoraError> {
        let mut remaining = self.remaining_failures.lock().unwrap();
        if *remaining > 0 {
            *remaining -= 1;
            return Err(AncoraError::Storage("injected store failure".into()));
        }
        self.inner.append(run_id, event)
    }

    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        self.inner.read(run_id)
    }

    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        self.inner.load(run_id, seq)
    }
}

struct SimpleActivity {
    key: String,
    counter: Arc<AtomicUsize>,
}

impl ancora_core::activity::Activity for SimpleActivity {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn execute(&self) -> Result<String, AncoraError> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        Ok(r#"{"ok":true}"#.into())
    }
}

fn record_activity(
    store: &dyn JournalStore,
    run_id: &str,
    counter: &Arc<AtomicUsize>,
    key: &str,
) -> Result<String, AncoraError> {
    use ancora_core::idempotency::{write_once, WriteActivity};
    let act = SimpleActivity {
        key: key.into(),
        counter: Arc::clone(counter),
    };
    let wa = WriteActivity::new(&act).unwrap();
    write_once(run_id, wa, store)
}

#[test]
fn store_failure_on_append_returns_error_without_corruption() {
    let store = FaultyStore::new(1);
    let run_id = "run-fail-append";
    let counter = Arc::new(AtomicUsize::new(0));

    // First call: store fails to append.
    let result = record_activity(&store, run_id, &counter, "act-1");
    assert!(result.is_err(), "expected store failure error");

    // Activity must have been executed (execute runs before append).
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    // Existing events are intact (none were written during the failure).
    let events = store.read(run_id).unwrap();
    assert!(
        events.is_empty(),
        "no event should be stored after append failure"
    );
}

#[test]
fn store_recovery_after_failure_records_correctly() {
    let store = FaultyStore::new(1);
    let run_id = "run-recover";
    let counter = Arc::new(AtomicUsize::new(0));

    // First call fails (activity executes but journal write fails).
    let _ = record_activity(&store, run_id, &counter, "act-1");
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    // Second call: store succeeds now.
    let result = record_activity(&store, run_id, &counter, "act-1");
    assert!(result.is_ok(), "second call should succeed");

    // Activity executed again because the first journal write failed.
    // This is acceptable: without a successful journal entry, retry is correct.
    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 1, "exactly one event should be stored");
}

#[test]
fn successful_appends_are_not_affected_by_earlier_failure() {
    let store = FaultyStore::new(1);
    let run_id = "run-partial";
    let c1 = Arc::new(AtomicUsize::new(0));
    let c2 = Arc::new(AtomicUsize::new(0));

    // act-1: store fails.
    let _ = record_activity(&store, run_id, &c1, "act-1");

    // act-2: store succeeds.
    let r2 = record_activity(&store, run_id, &c2, "act-2");
    assert!(r2.is_ok());

    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 1, "only act-2 is stored");

    // act-2 replay should not re-execute.
    let r2b = record_activity(&store, run_id, &c2, "act-2");
    assert!(r2b.is_ok());
    assert_eq!(c2.load(Ordering::SeqCst), 1, "act-2 must not re-execute");
}

#[test]
fn zero_failures_store_behaves_normally() {
    let store = FaultyStore::new(0);
    let run_id = "run-no-fail";
    let counter = Arc::new(AtomicUsize::new(0));

    let result = record_activity(&store, run_id, &counter, "act-1");
    assert!(result.is_ok());
    let events = store.read(run_id).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn read_always_succeeds_even_during_append_failures() {
    let store = FaultyStore::new(5);
    let run_id = "run-read-ok";
    let counter = Arc::new(AtomicUsize::new(0));

    for i in 0..3 {
        let _ = record_activity(&store, run_id, &counter, &format!("act-{}", i));
    }

    // Read must always succeed regardless of append failures.
    let events = store.read(run_id).unwrap();
    assert!(
        events.is_empty(),
        "no events stored when all appends failed"
    );
}
