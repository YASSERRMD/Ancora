//! Change log for the offline period.
//!
//! While the device is disconnected the change log records every mutation in
//! chronological order so the sync engine can replay them deterministically
//! when connectivity is restored.

use crate::model::{JournalEntry, SeqNo};
use serde::{Deserialize, Serialize};

/// The kind of change that occurred to a key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeKind {
    /// A new value was written.
    Put,
    /// A key was deleted.
    Delete,
    /// An existing value was updated in-place.
    Update,
}

/// One record in the change log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeRecord {
    /// The journal entry sequence number this change is derived from.
    pub journal_seq: SeqNo,
    /// The logical key that changed.
    pub key: String,
    /// What kind of change this was.
    pub kind: ChangeKind,
    /// Serialized before-value (empty for Put).
    pub before: Option<Vec<u8>>,
    /// Serialized after-value (empty for Delete).
    pub after: Option<Vec<u8>>,
}

/// Append-only change log covering the device's offline period.
pub struct ChangeLog {
    records: Vec<ChangeRecord>,
}

impl ChangeLog {
    /// Create an empty change log.
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    /// Record a Put change derived from a journal entry.
    pub fn record_put(&mut self, entry: &JournalEntry) {
        self.records.push(ChangeRecord {
            journal_seq: entry.seq,
            key: entry.key.clone(),
            kind: ChangeKind::Put,
            before: None,
            after: Some(entry.payload.clone()),
        });
    }

    /// Record a Delete change.
    pub fn record_delete(&mut self, journal_seq: SeqNo, key: impl Into<String>, before: Vec<u8>) {
        self.records.push(ChangeRecord {
            journal_seq,
            key: key.into(),
            kind: ChangeKind::Delete,
            before: Some(before),
            after: None,
        });
    }

    /// Record an Update change.
    pub fn record_update(
        &mut self,
        entry: &JournalEntry,
        before: Vec<u8>,
    ) {
        self.records.push(ChangeRecord {
            journal_seq: entry.seq,
            key: entry.key.clone(),
            kind: ChangeKind::Update,
            before: Some(before),
            after: Some(entry.payload.clone()),
        });
    }

    /// Return all records in order.
    pub fn records(&self) -> &[ChangeRecord] {
        &self.records
    }

    /// Return the number of recorded changes.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Return `true` when the change log is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Drain all records (e.g. after a successful sync).
    pub fn drain(&mut self) -> Vec<ChangeRecord> {
        std::mem::take(&mut self.records)
    }

    /// Replay the change log onto a simple key-value store (Vec<(String, Vec<u8>)>).
    /// Returns the resulting key-value pairs in insertion order.
    pub fn replay(&self, mut store: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
        for record in &self.records {
            match record.kind {
                ChangeKind::Put | ChangeKind::Update => {
                    if let Some(after) = &record.after {
                        if let Some(slot) = store.iter_mut().find(|(k, _)| k == &record.key) {
                            slot.1 = after.clone();
                        } else {
                            store.push((record.key.clone(), after.clone()));
                        }
                    }
                }
                ChangeKind::Delete => {
                    store.retain(|(k, _)| k != &record.key);
                }
            }
        }
        store
    }
}

impl Default for ChangeLog {
    fn default() -> Self {
        Self::new()
    }
}
