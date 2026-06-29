//! Local-first journal with sync markers.
//!
//! The journal is an append-only log of [`JournalEntry`] values stored
//! in memory (or, conceptually, on a local disk).  Entries are tagged with
//! a [`SyncMarker`] so the sync engine knows which entries still need to be
//! uploaded to the hub.

use crate::model::{JournalEntry, SeqNo, SyncId, SyncMarker};

/// In-memory local-first journal.
pub struct Journal {
    device_id: SyncId,
    entries: Vec<JournalEntry>,
    next_seq: SeqNo,
}

impl Journal {
    /// Create a new empty journal for the given device.
    pub fn new(device_id: impl Into<SyncId>) -> Self {
        Self {
            device_id: device_id.into(),
            entries: Vec::new(),
            next_seq: 1,
        }
    }

    /// Append a new pending entry and return its sequence number.
    pub fn append(&mut self, key: impl Into<String>, payload: Vec<u8>) -> SeqNo {
        let seq = self.next_seq;
        let entry = JournalEntry::new(seq, self.device_id.clone(), key, payload);
        self.entries.push(entry);
        self.next_seq += 1;
        seq
    }

    /// Return all entries that are still pending (not yet synced).
    pub fn pending_entries(&self) -> Vec<&JournalEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.marker, SyncMarker::Pending))
            .collect()
    }

    /// Mark an entry (by seq) as synced by the hub.
    pub fn mark_synced(&mut self, seq: SeqNo, hub_seq: SeqNo) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.seq == seq) {
            entry.mark_synced(hub_seq);
            true
        } else {
            false
        }
    }

    /// Mark an entry as in-flight.
    pub fn mark_in_flight(&mut self, seq: SeqNo, attempt: u32) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.seq == seq) {
            entry.mark_in_flight(attempt);
            true
        } else {
            false
        }
    }

    /// Return total number of entries in the journal.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` when the journal has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return all entries in sequence order.
    pub fn all_entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Retrieve a single entry by sequence number.
    pub fn get(&self, seq: SeqNo) -> Option<&JournalEntry> {
        self.entries.iter().find(|e| e.seq == seq)
    }
}
