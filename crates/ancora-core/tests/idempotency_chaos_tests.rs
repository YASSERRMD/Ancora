use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use ancora_core::{
    activity::Activity,
    error::AncoraError,
    idempotency::{write_once, WriteActivity},
    journal::{JournalStore, MemoryStore},
};
use proptest::prelude::*;

/// Activity that records how many times it was executed.
/// Simulates a side-effectful operation with a counter.
struct CountingActivity {
    key: String,
    result: String,
    call_count: Arc<AtomicUsize>,
}

impl Activity for CountingActivity {
    fn key(&self) -> String {
        self.key.clone()
    }
    fn execute(&self) -> Result<String, AncoraError> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        Ok(self.result.clone())
    }
}

/// Simulate a "crash" by only writing some events to the store.
/// `crash_after` specifies the number of write_once calls that succeed
/// before we stop (simulating a process kill mid-run).
fn run_with_simulated_crash(
    run_id: &str,
    keys: &[String],
    crash_after: usize,
    store: &dyn JournalStore,
    call_counts: &[Arc<AtomicUsize>],
) {
    for (i, key) in keys.iter().enumerate() {
        if i >= crash_after {
            break;
        }
        let activity = CountingActivity {
            key: key.clone(),
            result: format!("result-{}", i),
            call_count: Arc::clone(&call_counts[i]),
        };
        let wa = WriteActivity::new(&activity).unwrap();
        let _ = write_once(run_id, wa, store);
    }
}

/// Resume the run after "crash" -- all previously-journaled activities
/// must NOT be re-executed (idempotency). Remaining activities execute once.
fn resume_run(
    run_id: &str,
    keys: &[String],
    store: &dyn JournalStore,
    call_counts: &[Arc<AtomicUsize>],
) {
    for (i, key) in keys.iter().enumerate() {
        let activity = CountingActivity {
            key: key.clone(),
            result: format!("result-{}", i),
            call_count: Arc::clone(&call_counts[i]),
        };
        let wa = WriteActivity::new(&activity).unwrap();
        let _ = write_once(run_id, wa, store);
    }
}

proptest! {
    #[test]
    fn no_duplicate_side_effects_after_crash_and_resume(
        keys in proptest::collection::vec("[a-z]{4,10}", 1..8),
        crash_after in 0usize..8,
    ) {
        let run_id = "chaos-run";
        let store = MemoryStore::new();
        let call_counts: Vec<Arc<AtomicUsize>> = (0..keys.len())
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();

        let n = keys.len();
        let crash_at = crash_after.min(n);

        // First run -- crashes after `crash_at` activities.
        run_with_simulated_crash(run_id, &keys, crash_at, &store, &call_counts);

        // Resume -- must not re-execute already-journaled activities.
        resume_run(run_id, &keys, &store, &call_counts);

        // Every activity key must have been executed EXACTLY once.
        for (i, count) in call_counts.iter().enumerate() {
            let c = count.load(Ordering::SeqCst);
            prop_assert_eq!(
                c, 1,
                "activity {} ({}) must execute exactly once, got {}",
                i, keys[i], c
            );
        }
    }

    #[test]
    fn write_once_returns_same_result_on_repeated_calls(
        key in "[a-z]{4,10}",
        result in "[a-z0-9]{1,20}",
    ) {
        let run_id = "idem-run";
        let store = MemoryStore::new();
        let count = Arc::new(AtomicUsize::new(0));

        for _ in 0..5 {
            let activity = CountingActivity {
                key: key.clone(),
                result: result.clone(),
                call_count: Arc::clone(&count),
            };
            let wa = WriteActivity::new(&activity).unwrap();
            let r = write_once(run_id, wa, &store).unwrap();
            prop_assert_eq!(&r, &result, "result must be stable across repeated calls");
        }

        prop_assert_eq!(
            count.load(Ordering::SeqCst), 1,
            "activity must only execute once regardless of repeated calls"
        );
    }

    #[test]
    fn different_run_ids_are_isolated(
        key in "[a-z]{4,10}",
    ) {
        let store = MemoryStore::new();
        let count_a = Arc::new(AtomicUsize::new(0));
        let count_b = Arc::new(AtomicUsize::new(0));

        let act_a = CountingActivity {
            key: key.clone(), result: "a".into(), call_count: Arc::clone(&count_a),
        };
        let act_b = CountingActivity {
            key: key.clone(), result: "b".into(), call_count: Arc::clone(&count_b),
        };

        let _ = write_once("run-a", WriteActivity::new(&act_a).unwrap(), &store);
        let _ = write_once("run-b", WriteActivity::new(&act_b).unwrap(), &store);

        prop_assert_eq!(count_a.load(Ordering::SeqCst), 1, "run-a activity must execute once");
        prop_assert_eq!(count_b.load(Ordering::SeqCst), 1, "run-b activity must execute once");
    }
}
