//! Sync protocol between an edge device and the hub.
//!
//! The protocol is intentionally simple and transport-agnostic: the caller
//! serialises [`SyncRequest`] and [`SyncResponse`] however it likes (e.g.
//! over TCP, QUIC, or a Unix socket).  No I/O is performed here.

use crate::model::{JournalEntry, SeqNo, SyncId};
use serde::{Deserialize, Serialize};

/// A batch of entries the device wants to upload to the hub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncRequest {
    /// Identifier of the originating device.
    pub device_id: SyncId,
    /// Entries being uploaded.
    pub entries: Vec<JournalEntry>,
    /// Optional resume token from a previous partial sync.
    pub resume_token: Option<String>,
}

/// The hub's reply to a [`SyncRequest`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncResponse {
    /// Hub-assigned sequence numbers for each accepted entry (same order as request).
    pub acked_seqs: Vec<SeqNo>,
    /// Entries the hub is pushing back to the device (e.g. remote writes).
    pub remote_entries: Vec<JournalEntry>,
    /// Resume token for partial syncs; `None` means sync is complete.
    pub resume_token: Option<String>,
    /// Whether the hub detected any conflicts.
    pub has_conflicts: bool,
}

/// Minimal in-memory hub used in tests and as a reference implementation.
pub struct Hub {
    entries: Vec<JournalEntry>,
    next_hub_seq: SeqNo,
}

impl Hub {
    /// Create a new, empty hub.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_hub_seq: 1,
        }
    }

    /// Process an incoming [`SyncRequest`] and produce a [`SyncResponse`].
    ///
    /// Deduplication: entries whose (device_id, seq) pair is already known
    /// are silently skipped (idempotent upload).
    pub fn process(&mut self, request: SyncRequest) -> SyncResponse {
        let mut acked_seqs = Vec::new();
        let mut has_conflicts = false;

        for mut entry in request.entries {
            // Idempotency: skip if already stored.
            let already_stored = self
                .entries
                .iter()
                .any(|e| e.device_id == entry.device_id && e.seq == entry.seq);
            if already_stored {
                // Return the hub seq we assigned previously.
                if let Some(stored) = self
                    .entries
                    .iter()
                    .find(|e| e.device_id == entry.device_id && e.seq == entry.seq)
                {
                    if let crate::model::SyncMarker::Synced { hub_seq } = stored.marker {
                        acked_seqs.push(hub_seq);
                    }
                }
                continue;
            }

            // Conflict check: is there already an entry for this key from a
            // *different* device?
            if self
                .entries
                .iter()
                .any(|e| e.key == entry.key && e.device_id != entry.device_id)
            {
                has_conflicts = true;
            }

            let hub_seq = self.next_hub_seq;
            self.next_hub_seq += 1;
            entry.mark_synced(hub_seq);
            acked_seqs.push(hub_seq);
            self.entries.push(entry);
        }

        SyncResponse {
            acked_seqs,
            remote_entries: Vec::new(),
            resume_token: None,
            has_conflicts,
        }
    }

    /// Return all entries stored by the hub.
    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Return the total number of entries stored.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` when the hub has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
