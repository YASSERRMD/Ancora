use ancora_proto::ancora::{journal_event::Event, ActivityRecordedEvent, JournalEvent};

use crate::activity::Activity;
use crate::error::AncoraError;
use crate::journal::JournalStore;

/// A write-effect activity whose key must be non-empty.
///
/// Callers pass a `WriteActivity` to `write_once` instead of calling
/// `record_or_replay` directly, which enforces at the type level that
/// write operations always carry an idempotency key.
pub struct WriteActivity<'a> {
    inner: &'a dyn Activity,
}

impl<'a> WriteActivity<'a> {
    /// Wrap an `Activity`. Returns an error if `activity.key()` is empty.
    pub fn new(activity: &'a dyn Activity) -> Result<Self, AncoraError> {
        if activity.key().is_empty() {
            return Err(AncoraError::ToolInputInvalid {
                name: "<write-activity>".to_string(),
                reason: "write activities must have a non-empty idempotency key".to_string(),
            });
        }
        Ok(Self { inner: activity })
    }

    /// The idempotency key (guaranteed non-empty).
    pub fn key(&self) -> String {
        self.inner.key()
    }
}

/// Execute a write-effect activity at most once.
///
/// - If the journal already contains an `ActivityRecorded` event with the
///   matching key, return the stored result without calling `execute`.
/// - Otherwise call `execute`, record the result, and return it.
///
/// The short-circuit on retry means a crash after the side effect but before
/// the journal write cannot cause the effect to be applied twice: the next
/// attempt will find the journaled result and skip execution.
pub fn write_once(
    run_id: &str,
    activity: WriteActivity<'_>,
    store: &dyn JournalStore,
) -> Result<String, AncoraError> {
    let key = activity.key();

    for event in store.read(run_id)? {
        if let Some(Event::ActivityRecorded(ref recorded)) = event.event {
            if recorded.activity_key == key {
                return Ok(recorded.result_json.clone());
            }
        }
    }

    let result = activity.inner.execute()?;

    let journal_event = JournalEvent {
        event_id: key.clone(),
        run_id: run_id.to_string(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key,
            activity_kind: "write".to_string(),
            input_json: String::new(),
            result_json: result.clone(),
            replayed: false,
        })),
    };

    store.append(run_id, journal_event)?;
    Ok(result)
}

/// A compensating action paired with a write activity.
///
/// If the write succeeds but the broader transaction must be rolled back,
/// `compensate` is called once. It is also guarded by an idempotency key so
/// it cannot be applied twice.
pub struct CompensatingAction {
    /// Key of the write this action compensates.
    pub write_key: String,
    /// Function to call when rolling back. Returns a JSON result string.
    pub compensate: Box<dyn Fn() -> Result<String, AncoraError> + Send + Sync>,
}

/// Execute `action.compensate()` at most once.
///
/// Uses `<write_key>:compensate` as the journal key so the compensating action
/// is idempotent even if the rollback path is retried.
pub fn run_compensating_action(
    run_id: &str,
    action: &CompensatingAction,
    store: &dyn JournalStore,
) -> Result<String, AncoraError> {
    let key = format!("{}:compensate", action.write_key);

    for event in store.read(run_id)? {
        if let Some(Event::ActivityRecorded(ref recorded)) = event.event {
            if recorded.activity_key == key {
                return Ok(recorded.result_json.clone());
            }
        }
    }

    let result = (action.compensate)()?;

    let journal_event = JournalEvent {
        event_id: key.clone(),
        run_id: run_id.to_string(),
        seq: 0,
        recorded_at_ns: 0,
        event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
            activity_key: key,
            activity_kind: "compensate".to_string(),
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

    struct SimpleActivity {
        key: String,
        result: String,
        counter: Arc<AtomicUsize>,
    }

    impl Activity for SimpleActivity {
        fn execute(&self) -> Result<String, AncoraError> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            Ok(self.result.clone())
        }
        fn key(&self) -> String {
            self.key.clone()
        }
    }

    fn make(key: &str, result: &str, counter: Arc<AtomicUsize>) -> SimpleActivity {
        SimpleActivity {
            key: key.to_string(),
            result: result.to_string(),
            counter,
        }
    }

    #[test]
    fn empty_key_is_rejected() {
        let counter = Arc::new(AtomicUsize::new(0));
        let act = make("", r#""x""#, counter);
        assert!(WriteActivity::new(&act).is_err());
    }

    #[test]
    fn crash_between_effect_and_journal_does_not_double_apply() {
        let store = MemoryStore::new();
        let counter = Arc::new(AtomicUsize::new(0));

        let act = make("write-k1", r#""done""#, Arc::clone(&counter));
        let r1 = write_once("run-w1", WriteActivity::new(&act).unwrap(), &store).unwrap();
        assert_eq!(r1, r#""done""#);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        let act2 = make("write-k1", r#""ignored""#, Arc::clone(&counter));
        let r2 = write_once("run-w1", WriteActivity::new(&act2).unwrap(), &store).unwrap();
        assert_eq!(r2, r#""done""#, "must return journaled result on retry");
        assert_eq!(counter.load(Ordering::SeqCst), 1, "must not re-execute");
    }

    #[test]
    fn compensating_action_runs_on_rollback() {
        let store = MemoryStore::new();
        let comp_counter = Arc::new(AtomicUsize::new(0));

        let cc = Arc::clone(&comp_counter);
        let action = CompensatingAction {
            write_key: "write-k2".to_string(),
            compensate: Box::new(move || {
                cc.fetch_add(1, Ordering::SeqCst);
                Ok(r#""compensated""#.to_string())
            }),
        };

        let r = run_compensating_action("run-w2", &action, &store).unwrap();
        assert_eq!(r, r#""compensated""#);
        assert_eq!(comp_counter.load(Ordering::SeqCst), 1);

        let r2 = run_compensating_action("run-w2", &action, &store).unwrap();
        assert_eq!(r2, r#""compensated""#, "replay must return journaled result");
        assert_eq!(
            comp_counter.load(Ordering::SeqCst),
            1,
            "must not re-run compensator"
        );
    }

    #[test]
    fn compensating_key_is_distinct_from_write_key() {
        let store = MemoryStore::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let comp_counter = Arc::new(AtomicUsize::new(0));

        let act = make("write-k3", r#""done""#, Arc::clone(&counter));
        write_once("run-w3", WriteActivity::new(&act).unwrap(), &store).unwrap();

        let cc = Arc::clone(&comp_counter);
        let action = CompensatingAction {
            write_key: "write-k3".to_string(),
            compensate: Box::new(move || {
                cc.fetch_add(1, Ordering::SeqCst);
                Ok(r#""undone""#.to_string())
            }),
        };
        run_compensating_action("run-w3", &action, &store).unwrap();

        let events = store.read("run-w3").unwrap();
        let keys: Vec<String> = events
            .iter()
            .filter_map(|e| {
                if let Some(Event::ActivityRecorded(ref a)) = e.event {
                    Some(a.activity_key.clone())
                } else {
                    None
                }
            })
            .collect();

        assert!(keys.contains(&"write-k3".to_string()));
        assert!(keys.contains(&"write-k3:compensate".to_string()));
    }
}
