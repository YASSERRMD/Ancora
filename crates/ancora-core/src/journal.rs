use ancora_proto::ancora::JournalEvent;

use crate::error::AncoraError;

/// Opaque checkpoint blob saved at the end of each node so a run can
/// resume from a known-good point without replaying the full journal.
///
/// The bytes are engine-internal; no schema is imposed by this trait.
pub trait CheckpointStore: Send + Sync {
    /// Persist a checkpoint for a run at the given sequence number.
    /// Overwrites any prior checkpoint for the same run.
    fn save(&self, run_id: &str, at_seq: u64, data: &[u8]) -> Result<(), AncoraError>;

    /// Load the most recent checkpoint for a run.
    /// Returns `None` if no checkpoint has been saved yet.
    fn load_checkpoint(&self, run_id: &str) -> Result<Option<(u64, Vec<u8>)>, AncoraError>;
}

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
