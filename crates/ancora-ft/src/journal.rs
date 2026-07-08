//! Journaled adapter selection: records all selection events and supports replay.

use crate::model::AdapterId;
use crate::runtime::TenantAdapterMap;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A single adapter selection event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionEvent {
    /// Monotonically increasing sequence number.
    pub seq: u64,
    /// Unix timestamp in seconds.
    pub timestamp: u64,
    /// Tenant that made the selection.
    pub tenant_id: String,
    /// The adapter selected. None means assignment was cleared.
    pub adapter_id: Option<AdapterId>,
}

impl SelectionEvent {
    pub fn new(seq: u64, tenant_id: impl Into<String>, adapter_id: Option<AdapterId>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        SelectionEvent {
            seq,
            timestamp,
            tenant_id: tenant_id.into(),
            adapter_id,
        }
    }
}

/// The adapter selection journal: append-only log of selection events.
#[derive(Debug, Clone, Default)]
pub struct SelectionJournal {
    events: Vec<SelectionEvent>,
    next_seq: u64,
}

impl SelectionJournal {
    pub fn new() -> Self {
        SelectionJournal {
            events: Vec::new(),
            next_seq: 0,
        }
    }

    /// Record a selection event.
    pub fn record(&mut self, tenant_id: impl Into<String>, adapter_id: Option<AdapterId>) -> u64 {
        let seq = self.next_seq;
        self.events
            .push(SelectionEvent::new(seq, tenant_id, adapter_id));
        self.next_seq += 1;
        seq
    }

    /// All recorded events in sequence order.
    pub fn events(&self) -> &[SelectionEvent] {
        &self.events
    }

    /// Replay all events onto a fresh TenantAdapterMap, returning the
    /// reconstructed state.
    pub fn replay(&self) -> TenantAdapterMap {
        let mut map = TenantAdapterMap::new();
        for event in &self.events {
            match &event.adapter_id {
                Some(id) => map.assign(event.tenant_id.clone(), id.clone()),
                None => {
                    map.remove(&event.tenant_id);
                }
            }
        }
        map
    }

    /// Total number of events recorded.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Retrieve events for a specific tenant.
    pub fn events_for_tenant(&self, tenant_id: &str) -> Vec<&SelectionEvent> {
        self.events
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
}

/// Select an adapter for a tenant, recording the event in the journal.
pub fn select_for_tenant(
    journal: &mut SelectionJournal,
    map: &mut TenantAdapterMap,
    tenant_id: impl Into<String>,
    adapter_id: AdapterId,
) -> u64 {
    let tid: String = tenant_id.into();
    map.assign(tid.clone(), adapter_id.clone());
    journal.record(tid, Some(adapter_id))
}

/// Clear an adapter assignment for a tenant, recording the event.
pub fn clear_for_tenant(
    journal: &mut SelectionJournal,
    map: &mut TenantAdapterMap,
    tenant_id: impl Into<String>,
) -> u64 {
    let tid: String = tenant_id.into();
    map.remove(&tid);
    journal.record(tid, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AdapterId;

    #[test]
    fn journal_records_and_replays() {
        let mut journal = SelectionJournal::new();
        let mut map = TenantAdapterMap::new();

        select_for_tenant(&mut journal, &mut map, "tenant-1", AdapterId::new("a1"));
        select_for_tenant(&mut journal, &mut map, "tenant-2", AdapterId::new("a2"));

        assert_eq!(journal.len(), 2);

        let replayed = journal.replay();
        assert_eq!(replayed.get("tenant-1").unwrap().as_str(), "a1");
        assert_eq!(replayed.get("tenant-2").unwrap().as_str(), "a2");
    }

    #[test]
    fn journal_replay_overwrites_with_latest() {
        let mut journal = SelectionJournal::new();
        let mut map = TenantAdapterMap::new();

        select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a1"));
        select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a2"));

        let replayed = journal.replay();
        assert_eq!(replayed.get("t1").unwrap().as_str(), "a2");
    }

    #[test]
    fn journal_replay_handles_clear() {
        let mut journal = SelectionJournal::new();
        let mut map = TenantAdapterMap::new();

        select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a1"));
        clear_for_tenant(&mut journal, &mut map, "t1");

        let replayed = journal.replay();
        assert!(replayed.get("t1").is_none());
    }

    #[test]
    fn journal_events_for_tenant() {
        let mut journal = SelectionJournal::new();
        let mut map = TenantAdapterMap::new();

        select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a1"));
        select_for_tenant(&mut journal, &mut map, "t2", AdapterId::new("a2"));
        select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a3"));

        let t1_events = journal.events_for_tenant("t1");
        assert_eq!(t1_events.len(), 2);
    }

    #[test]
    fn journal_sequence_monotonic() {
        let mut journal = SelectionJournal::new();
        let mut map = TenantAdapterMap::new();

        let s1 = select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a1"));
        let s2 = select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a2"));
        let s3 = select_for_tenant(&mut journal, &mut map, "t1", AdapterId::new("a3"));

        assert!(s1 < s2);
        assert!(s2 < s3);
    }
}
