//! Conflict detection and resolution for concurrent edits.
//!
//! A conflict occurs when two devices write different payloads to the same
//! key.  [`ConflictDetector`] identifies conflicts from a set of entries,
//! and [`ConflictPolicy`] decides how to resolve them.

use crate::model::{Conflict, JournalEntry, ResolutionOutcome};

/// Detects conflicts in a flat list of journal entries.
pub struct ConflictDetector;

impl ConflictDetector {
    /// Scan `entries` for pairs that target the same key but originate from
    /// different devices or carry different payloads.
    pub fn detect(entries: &[JournalEntry]) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        for (i, a) in entries.iter().enumerate() {
            for b in entries.iter().skip(i + 1) {
                if a.key == b.key && (a.device_id != b.device_id || a.payload != b.payload) {
                    conflicts.push(Conflict {
                        key: a.key.clone(),
                        local: a.clone(),
                        remote: b.clone(),
                    });
                }
            }
        }

        conflicts
    }

    /// Return `true` if any conflicts exist in `entries`.
    pub fn has_conflicts(entries: &[JournalEntry]) -> bool {
        !Self::detect(entries).is_empty()
    }
}

/// Policy used to resolve a detected conflict.
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictPolicy {
    /// The entry with the higher sequence number wins.
    LastWriteWins,
    /// Always keep the local (lower hub seq or local seq) version.
    PreferLocal,
    /// Always accept the remote version.
    PreferRemote,
    /// Concatenate both payloads (useful for CRDT-style append sets).
    Merge,
}

impl ConflictPolicy {
    /// Apply this policy to a single conflict and return the resolution outcome.
    pub fn resolve(&self, conflict: &Conflict) -> ResolutionOutcome {
        match self {
            ConflictPolicy::LastWriteWins => {
                if conflict.local.seq >= conflict.remote.seq {
                    ResolutionOutcome::KeepLocal
                } else {
                    ResolutionOutcome::AcceptRemote
                }
            }
            ConflictPolicy::PreferLocal => ResolutionOutcome::KeepLocal,
            ConflictPolicy::PreferRemote => ResolutionOutcome::AcceptRemote,
            ConflictPolicy::Merge => {
                let mut merged_payload = conflict.local.payload.clone();
                merged_payload.extend_from_slice(&conflict.remote.payload);
                let checksum = crate::model::compute_checksum(&merged_payload);
                let merged_entry = JournalEntry {
                    seq: conflict.local.seq.max(conflict.remote.seq) + 1,
                    timestamp: conflict.local.timestamp.max(conflict.remote.timestamp),
                    device_id: format!(
                        "{}-{}-merged",
                        conflict.local.device_id, conflict.remote.device_id
                    ),
                    key: conflict.key.clone(),
                    payload: merged_payload,
                    marker: crate::model::SyncMarker::Pending,
                    checksum,
                };
                ResolutionOutcome::Merged(merged_entry)
            }
        }
    }

    /// Resolve all conflicts in a list and return the winning entries.
    pub fn resolve_all(&self, conflicts: &[Conflict]) -> Vec<JournalEntry> {
        conflicts
            .iter()
            .map(|c| match self.resolve(c) {
                ResolutionOutcome::KeepLocal => c.local.clone(),
                ResolutionOutcome::AcceptRemote => c.remote.clone(),
                ResolutionOutcome::Merged(e) => e,
            })
            .collect()
    }
}
