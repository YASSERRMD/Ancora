//! Residency-aware sync: ensure data is only transferred to permitted zones.
//!
//! Each [`JournalEntry`] may be tagged with a [`ResidencyZone`].  Before
//! syncing, the [`ResidencyFilter`] checks whether the destination hub zone
//! matches the entry's residency policy.

use crate::model::{JournalEntry, ResidencyZone};
use crate::protocol::SyncRequest;

/// Tags an entry with its required residency zone.
///
/// In a real system this metadata might live in the entry itself or in a
/// separate policy store.  Here we use a side-table for simplicity.
#[derive(Debug, Clone)]
pub struct ResidencyTag {
    /// Journal entry sequence number.
    pub seq: u64,
    /// Required residency zone.
    pub zone: ResidencyZone,
}

/// Filters a set of entries based on where the hub resides.
pub struct ResidencyFilter {
    /// Zone of the hub we are about to sync with.
    hub_zone: ResidencyZone,
    /// Tags for entries we intend to upload.
    tags: Vec<ResidencyTag>,
}

impl ResidencyFilter {
    /// Create a new filter for a hub in the given zone.
    pub fn new(hub_zone: ResidencyZone, tags: Vec<ResidencyTag>) -> Self {
        Self { hub_zone, tags }
    }

    /// Return `true` if an entry with the given zone tag is allowed to be
    /// sent to the hub zone.
    pub fn is_allowed(&self, entry_zone: &ResidencyZone) -> bool {
        match entry_zone {
            ResidencyZone::Global => true,
            ResidencyZone::Region(r) => match &self.hub_zone {
                ResidencyZone::Region(h) => r == h,
                ResidencyZone::Global => false, // global hub cannot satisfy a specific region requirement
                ResidencyZone::Local => false,
            },
            ResidencyZone::Local => false, // local-only data must never leave the device
        }
    }

    /// Filter `entries` to only those that are allowed to be uploaded.
    pub fn filter_allowed<'a>(&self, entries: &'a [JournalEntry]) -> Vec<&'a JournalEntry> {
        entries
            .iter()
            .filter(|e| {
                // Look up the tag; default to Global if no tag is present.
                let zone = self.tags
                    .iter()
                    .find(|t| t.seq == e.seq)
                    .map(|t| &t.zone)
                    .unwrap_or(&ResidencyZone::Global);
                self.is_allowed(zone)
            })
            .collect()
    }

    /// Build a [`SyncRequest`] containing only the residency-allowed entries.
    pub fn build_request(&self, device_id: impl Into<String>, entries: &[JournalEntry]) -> SyncRequest {
        let allowed: Vec<JournalEntry> = self
            .filter_allowed(entries)
            .into_iter()
            .cloned()
            .collect();
        SyncRequest {
            device_id: device_id.into(),
            entries: allowed,
            resume_token: None,
        }
    }
}
