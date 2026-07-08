use ancora_proto::ancora::{journal_event::Event, ActivityRecordedEvent, JournalEvent};

use crate::error::AncoraError;
use crate::journal::JournalStore;

/// A single non-deterministic unit of work that must be recorded in the
/// journal on first execution and replayed from the journal on subsequent
/// executions of the same run.
///
/// All implementations must be idempotent with respect to their key: two
/// activities with the same key must produce the same result or the second
/// must be satisfied by the journaled result of the first.
pub trait Activity: Send + Sync {
    /// Execute the activity and return a JSON-encoded result string.
    fn execute(&self) -> Result<String, AncoraError>;

    /// A unique, stable key for this activity within a run.
    ///
    /// The key is used as the idempotency key in the journal. It must:
    /// - Be the same every time this activity would be executed in the same
    ///   position during a fresh run.
    /// - Be different from any other activity in the same run.
    fn key(&self) -> String;
}

/// Execute `activity` at most once for `run_id`.
///
/// - **Fresh run**: `store` has no event with `activity.key()`. Call
///   `activity.execute()`, record the result as an `ActivityRecorded` event,
///   and return the result.
/// - **Replay**: `store` already has an event with `activity.key()`. Return the
///   journaled `result_json` directly without calling `execute()`.
pub fn record_or_replay(
    run_id: &str,
    activity: &dyn Activity,
    store: &dyn JournalStore,
) -> Result<String, AncoraError> {
    let key = activity.key();

    // Search for an existing journaled result for this key.
    for event in store.read(run_id)? {
        if let Some(Event::ActivityRecorded(ref recorded)) = event.event {
            if recorded.activity_key == key {
                return Ok(recorded.result_json.clone());
            }
        }
    }

    // No journal entry found: execute and record.
    let result = activity.execute()?;

    let journal_event = JournalEvent {
        event_id: key.clone(),
        run_id: run_id.to_string(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key,
            activity_kind: "activity".to_string(),
            input_json: String::new(),
            result_json: result.clone(),
            replayed: false,
        })),
    };

    store.append(run_id, journal_event)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::*;
    use crate::journal::MemoryStore;

    struct CountingActivity {
        counter: Arc<AtomicUsize>,
        key: String,
        result: String,
    }

    impl Activity for CountingActivity {
        fn execute(&self) -> Result<String, AncoraError> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            Ok(self.result.clone())
        }
        fn key(&self) -> String {
            self.key.clone()
        }
    }

    fn activity(key: &str, result: &str, counter: Arc<AtomicUsize>) -> CountingActivity {
        CountingActivity {
            counter,
            key: key.to_string(),
            result: result.to_string(),
        }
    }

    #[test]
    fn activity_executes_once_and_replays_from_journal() {
        let store = MemoryStore::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let r1 = record_or_replay(
            "run-1",
            &activity("key-a", r#""hello""#, Arc::clone(&counter)),
            &store,
        )
        .unwrap();
        assert_eq!(r1, r#""hello""#);
        assert_eq!(counter.load(Ordering::SeqCst), 1, "must execute once");

        let r2 = record_or_replay(
            "run-1",
            &activity("key-a", r#""ignored""#, Arc::clone(&counter)),
            &store,
        )
        .unwrap();
        assert_eq!(r2, r#""hello""#, "replay must return journaled result");
        assert_eq!(counter.load(Ordering::SeqCst), 1, "must not execute again");
    }

    #[test]
    fn replay_never_calls_the_underlying_activity() {
        let store = MemoryStore::new();
        let counter = Arc::new(AtomicUsize::new(0));

        record_or_replay(
            "run-x",
            &activity("step-1", r#""result""#, Arc::clone(&counter)),
            &store,
        )
        .unwrap();

        for _ in 0..10 {
            let r = record_or_replay(
                "run-x",
                &activity("step-1", r#""should-not-matter""#, Arc::clone(&counter)),
                &store,
            )
            .unwrap();
            assert_eq!(r, r#""result""#);
        }

        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "underlying activity must be called exactly once regardless of replay count"
        );
    }

    #[test]
    fn different_keys_execute_independently() {
        let store = MemoryStore::new();
        let c1 = Arc::new(AtomicUsize::new(0));
        let c2 = Arc::new(AtomicUsize::new(0));

        record_or_replay("run-2", &activity("k1", r#""a""#, Arc::clone(&c1)), &store).unwrap();
        record_or_replay("run-2", &activity("k2", r#""b""#, Arc::clone(&c2)), &store).unwrap();

        assert_eq!(c1.load(Ordering::SeqCst), 1);
        assert_eq!(c2.load(Ordering::SeqCst), 1);

        record_or_replay("run-2", &activity("k1", r#""a""#, Arc::clone(&c1)), &store).unwrap();
        record_or_replay("run-2", &activity("k2", r#""b""#, Arc::clone(&c2)), &store).unwrap();

        assert_eq!(
            c1.load(Ordering::SeqCst),
            1,
            "k1 replay must not re-execute"
        );
        assert_eq!(
            c2.load(Ordering::SeqCst),
            1,
            "k2 replay must not re-execute"
        );
    }
}
