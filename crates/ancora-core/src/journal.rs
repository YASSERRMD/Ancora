use ancora_proto::ancora::JournalEvent;

use crate::error::AncoraError;

/// Durable, ordered storage for journal events.
///
/// Implementations must guarantee:
/// - Events returned by `read` are ordered by `seq` ascending.
/// - `append` is atomic per event: partial writes are not visible.
/// - Duplicate `activity_key` on `ActivityRecorded` events must be rejected
///   with `AncoraError::JournalWrite`.
pub trait JournalStore: Send + Sync {
    /// Append a single event. Returns the assigned sequence number.
    fn append(&self, run_id: &str, event: JournalEvent) -> Result<u64, AncoraError>;

    /// Read all events for a run in seq-ascending order.
    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError>;

    /// Load a single event by run and sequence number.
    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError>;
}
