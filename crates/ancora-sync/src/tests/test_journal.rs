//! Tests: offline changes recorded in the journal.

use crate::journal::Journal;
use crate::model::SyncMarker;

#[test]
fn test_offline_changes_recorded() {
    let mut journal = Journal::new("device-1");
    let seq = journal.append("key/a", b"value-a".to_vec());
    assert_eq!(seq, 1);
    let entry = journal.get(seq).expect("entry should exist");
    assert_eq!(entry.key, "key/a");
    assert_eq!(entry.payload, b"value-a");
    assert_eq!(entry.marker, SyncMarker::Pending);
}

#[test]
fn test_journal_multiple_entries() {
    let mut journal = Journal::new("device-2");
    for i in 0..5u64 {
        journal.append(format!("key/{i}"), vec![i as u8]);
    }
    assert_eq!(journal.len(), 5);
    assert_eq!(journal.pending_entries().len(), 5);
}

#[test]
fn test_journal_mark_synced() {
    let mut journal = Journal::new("device-3");
    let seq = journal.append("key/x", b"data".to_vec());
    let ok = journal.mark_synced(seq, 42);
    assert!(ok);
    let entry = journal.get(seq).unwrap();
    assert_eq!(entry.marker, SyncMarker::Synced { hub_seq: 42 });
    assert_eq!(journal.pending_entries().len(), 0);
}

#[test]
fn test_journal_checksum_valid() {
    let mut journal = Journal::new("device-4");
    let seq = journal.append("key/y", b"hello world".to_vec());
    let entry = journal.get(seq).unwrap();
    assert!(entry.verify_checksum());
}
