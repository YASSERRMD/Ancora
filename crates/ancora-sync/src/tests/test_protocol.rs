//! Tests: sync uploads changes to the hub.

use crate::journal::Journal;
use crate::model::SyncMarker;
use crate::protocol::{Hub, SyncRequest};

#[test]
fn test_sync_uploads_changes_to_hub() {
    let mut journal = Journal::new("device-a");
    let seq = journal.append("config/x", b"42".to_vec());
    let entry = journal.get(seq).unwrap().clone();

    let mut hub = Hub::new();
    let request = SyncRequest {
        device_id: "device-a".into(),
        entries: vec![entry],
        resume_token: None,
    };
    let response = hub.process(request);

    assert_eq!(response.acked_seqs.len(), 1);
    assert!(!response.has_conflicts);
    assert_eq!(hub.len(), 1);
}

#[test]
fn test_hub_assigns_hub_seqs() {
    let mut journal = Journal::new("device-b");
    let s1 = journal.append("a", b"1".to_vec());
    let s2 = journal.append("b", b"2".to_vec());
    let entries = vec![
        journal.get(s1).unwrap().clone(),
        journal.get(s2).unwrap().clone(),
    ];

    let mut hub = Hub::new();
    let resp = hub.process(SyncRequest {
        device_id: "device-b".into(),
        entries,
        resume_token: None,
    });
    assert_eq!(resp.acked_seqs, vec![1, 2]);
}

#[test]
fn test_hub_stored_entries_are_synced() {
    let mut journal = Journal::new("device-c");
    let seq = journal.append("key/z", b"z-data".to_vec());
    let entry = journal.get(seq).unwrap().clone();

    let mut hub = Hub::new();
    hub.process(SyncRequest {
        device_id: "device-c".into(),
        entries: vec![entry],
        resume_token: None,
    });

    let stored = &hub.entries()[0];
    assert!(matches!(stored.marker, SyncMarker::Synced { .. }));
}
