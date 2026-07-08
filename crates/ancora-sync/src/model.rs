//! Core data model for offline sync and reconciliation.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A monotonic sequence number used for ordering journal entries.
pub type SeqNo = u64;

/// Unique identifier for an entry, device, or hub.
pub type SyncId = String;

/// Sync marker embedded in each journal entry to track sync state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncMarker {
    /// Entry has not yet been uploaded to the hub.
    Pending,
    /// Entry is currently being synced (in-flight).
    InFlight { attempt: u32 },
    /// Entry has been acknowledged by the hub.
    Synced { hub_seq: SeqNo },
}

/// A single entry in the local-first journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Sequence number (local, monotonically increasing).
    pub seq: SeqNo,
    /// Wall-clock timestamp (seconds since UNIX epoch).
    pub timestamp: u64,
    /// Device that produced this entry.
    pub device_id: SyncId,
    /// Logical key for the data being written.
    pub key: String,
    /// Serialized payload.
    pub payload: Vec<u8>,
    /// Current sync marker.
    pub marker: SyncMarker,
    /// Content hash for integrity verification.
    pub checksum: u64,
}

impl JournalEntry {
    /// Create a new pending journal entry.
    pub fn new(
        seq: SeqNo,
        device_id: impl Into<String>,
        key: impl Into<String>,
        payload: Vec<u8>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let checksum = compute_checksum(&payload);
        Self {
            seq,
            timestamp,
            device_id: device_id.into(),
            key: key.into(),
            payload,
            marker: SyncMarker::Pending,
            checksum,
        }
    }

    /// Mark this entry as synced by the hub.
    pub fn mark_synced(&mut self, hub_seq: SeqNo) {
        self.marker = SyncMarker::Synced { hub_seq };
    }

    /// Mark this entry as in-flight.
    pub fn mark_in_flight(&mut self, attempt: u32) {
        self.marker = SyncMarker::InFlight { attempt };
    }

    /// Check whether the payload matches the stored checksum.
    pub fn verify_checksum(&self) -> bool {
        compute_checksum(&self.payload) == self.checksum
    }
}

/// Compute a simple FNV-1a 64-bit checksum of a byte slice.
pub fn compute_checksum(data: &[u8]) -> u64 {
    const FNV_PRIME: u64 = 1_099_511_628_211;
    const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
    let mut hash = FNV_OFFSET;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// The residency zone to which data belongs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResidencyZone {
    /// Data may be stored anywhere.
    Global,
    /// Data must stay within a named geographic region.
    Region(String),
    /// Data must remain on the originating device.
    Local,
}

/// A conflict that arose when merging entries with the same key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    /// The key in dispute.
    pub key: String,
    /// The local entry that conflicted.
    pub local: JournalEntry,
    /// The remote entry that conflicted.
    pub remote: JournalEntry,
}

/// Outcome of applying a conflict-resolution policy.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionOutcome {
    /// Keep the local version.
    KeepLocal,
    /// Accept the remote version.
    AcceptRemote,
    /// Merge both into a new entry.
    Merged(JournalEntry),
}
