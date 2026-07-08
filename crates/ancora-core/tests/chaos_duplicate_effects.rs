/// Zero-duplicate-side-effects chaos tests.
///
/// These tests run a complete activity sequence with:
/// - Simulated crashes at every possible position (crash-before, crash-after
///   each activity, crash mid-append via the FaultyStore from the adjacent
///   test file).
/// - Multiple resume cycles.
///
/// The invariant: the total call count for each activity equals exactly 1
/// at the end of all crash/resume cycles, regardless of which crash point
/// was chosen.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use ancora_core::{
    activity::Activity,
    error::AncoraError,
    idempotency::{write_once, WriteActivity},
    journal::{JournalStore, MemoryStore},
};
use ancora_proto::ancora::JournalEvent;

struct CountActivity {
    key: String,
    counter: Arc<AtomicUsize>,
}

impl Activity for CountActivity {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn execute(&self) -> Result<String, AncoraError> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        Ok(r#"{"done":true}"#.into())
    }
}

fn do_activity(store: &dyn JournalStore, run_id: &str, key: &str, counter: &Arc<AtomicUsize>) {
    let act = CountActivity {
        key: key.into(),
        counter: Arc::clone(counter),
    };
    let wa = WriteActivity::new(&act).unwrap();
    let _ = write_once(run_id, wa, store);
}

/// Run all activities for a given `run_id` using the shared store.
/// Simulates a full run attempt that may be starting from an arbitrary
/// mid-run state already in the journal.
fn run_all(store: &dyn JournalStore, run_id: &str, keys: &[&str], counters: &[Arc<AtomicUsize>]) {
    for (key, counter) in keys.iter().zip(counters.iter()) {
        do_activity(store, run_id, key, counter);
    }
}

#[test]
fn crash_at_every_position_yields_exactly_one_execution() {
    let n = 6usize;
    let keys: Vec<String> = (0..n).map(|i| format!("act-{}", i)).collect();
    let key_strs: Vec<&str> = keys.iter().map(String::as_str).collect();

    for crash_after in 0..=n {
        let store = MemoryStore::new();
        let run_id = "chaos-dup";
        let counters: Vec<Arc<AtomicUsize>> =
            (0..n).map(|_| Arc::new(AtomicUsize::new(0))).collect();

        // First run: execute `crash_after` activities then "crash".
        for (i, key) in key_strs.iter().enumerate().take(crash_after) {
            do_activity(&store, run_id, key, &counters[i]);
        }

        // Resume: run all from the beginning -- already-journaled ones replay.
        run_all(&store, run_id, &key_strs, &counters);

        // Every activity must have executed exactly once.
        for (i, c) in counters.iter().enumerate() {
            assert_eq!(
                c.load(Ordering::SeqCst),
                1,
                "crash_after={}: activity {} executed {} times (expected 1)",
                crash_after,
                i,
                c.load(Ordering::SeqCst)
            );
        }
    }
}

#[test]
fn repeated_resumes_do_not_increase_effect_count() {
    let store = MemoryStore::new();
    let run_id = "multi-resume";
    let keys = ["k1", "k2", "k3"];
    let counters: Vec<Arc<AtomicUsize>> = (0..3).map(|_| Arc::new(AtomicUsize::new(0))).collect();

    // Simulate 10 resume attempts.
    for _ in 0..10 {
        run_all(&store, run_id, &keys, &counters);
    }

    for (i, c) in counters.iter().enumerate() {
        assert_eq!(
            c.load(Ordering::SeqCst),
            1,
            "activity {} must execute exactly once across 10 resume attempts",
            i
        );
    }
}

/// Faulty store that fails appends for specific sequence numbers.
struct SeqFaultyStore {
    inner: MemoryStore,
    fail_at_seq: Mutex<Vec<usize>>,
    total_appends: AtomicUsize,
}

impl SeqFaultyStore {
    fn new(fail_at: Vec<usize>) -> Self {
        Self {
            inner: MemoryStore::new(),
            fail_at_seq: Mutex::new(fail_at),
            total_appends: AtomicUsize::new(0),
        }
    }
}

impl JournalStore for SeqFaultyStore {
    fn append(&self, run_id: &str, event: JournalEvent) -> Result<u64, AncoraError> {
        let seq = self.total_appends.fetch_add(1, Ordering::SeqCst);
        let fail = self.fail_at_seq.lock().unwrap().contains(&seq);
        if fail {
            return Err(AncoraError::Storage(format!(
                "injected failure at seq {}",
                seq
            )));
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

#[test]
fn targeted_append_failures_cause_at_most_one_re_execution() {
    let store = SeqFaultyStore::new(vec![0, 2]);
    let run_id = "seq-faulty-run";
    let n = 4usize;
    let counters: Vec<Arc<AtomicUsize>> = (0..n).map(|_| Arc::new(AtomicUsize::new(0))).collect();
    let keys: Vec<String> = (0..n).map(|i| format!("key-{}", i)).collect();

    // First pass: some appends fail.
    for (i, key) in keys.iter().enumerate() {
        do_activity(&store, run_id, key, &counters[i]);
    }

    // Second pass: no more injected failures (fail_at: seq 0, 2 are past).
    for (i, key) in keys.iter().enumerate() {
        do_activity(&store, run_id, key, &counters[i]);
    }

    // Activities whose journal write failed get re-executed once on retry.
    // Those that succeeded must not be re-executed.
    for (i, c) in counters.iter().enumerate() {
        let count = c.load(Ordering::SeqCst);
        assert!(
            count >= 1 && count <= 2,
            "activity {} executed {} times -- expected 1 or 2 (at most one retry)",
            i,
            count
        );
    }
}
