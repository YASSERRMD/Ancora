//! Tests: change log records offline mutations.

use crate::changelog::{ChangeKind, ChangeLog};
use crate::journal::Journal;

#[test]
fn test_changelog_records_put() {
    let mut journal = Journal::new("dev-1");
    let seq = journal.append("key/a", b"val".to_vec());
    let entry = journal.get(seq).unwrap();

    let mut log = ChangeLog::new();
    log.record_put(entry);

    assert_eq!(log.len(), 1);
    assert_eq!(log.records()[0].kind, ChangeKind::Put);
    assert_eq!(log.records()[0].after.as_deref(), Some(b"val".as_ref()));
}

#[test]
fn test_changelog_records_delete() {
    let mut log = ChangeLog::new();
    log.record_delete(1, "key/b", b"old".to_vec());
    assert_eq!(log.records()[0].kind, ChangeKind::Delete);
    assert_eq!(log.records()[0].before.as_deref(), Some(b"old".as_ref()));
}

#[test]
fn test_changelog_replay() {
    let mut journal = Journal::new("dev-2");
    let seq_a = journal.append("key/a", b"v1".to_vec());
    let seq_b = journal.append("key/b", b"v2".to_vec());

    let mut log = ChangeLog::new();
    log.record_put(journal.get(seq_a).unwrap());
    log.record_put(journal.get(seq_b).unwrap());

    let store = log.replay(vec![]);
    assert_eq!(store.len(), 2);
    assert!(store.iter().any(|(k, v)| k == "key/a" && v == b"v1"));
    assert!(store.iter().any(|(k, v)| k == "key/b" && v == b"v2"));
}

#[test]
fn test_changelog_drain() {
    let mut log = ChangeLog::new();
    log.record_delete(1, "key/x", b"old".to_vec());
    let drained = log.drain();
    assert_eq!(drained.len(), 1);
    assert!(log.is_empty());
}
