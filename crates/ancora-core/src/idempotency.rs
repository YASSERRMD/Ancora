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
