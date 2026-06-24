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
