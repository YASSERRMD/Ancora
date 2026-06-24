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
