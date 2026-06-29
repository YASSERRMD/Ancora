//! Tests: determinism is preserved across sync (same inputs = same outputs).

use crate::changelog::ChangeLog;
use crate::journal::Journal;
use crate::model::compute_checksum;
use crate::protocol::{Hub, SyncRequest};

#[test]
fn test_determinism_preserved_across_sync() {
    // Build the same journal twice and ensure hub state is identical.
    fn build_hub(entries_data: &[(&str, &[u8])]) -> Vec<u8> {
        let mut journal = Journal::new("det-device");
        for (key, val) in entries_data {
            journal.append(*key, val.to_vec());
        }
        let entries = journal.all_entries().to_vec();
        let mut hub = Hub::new();
        hub.process(SyncRequest {
            device_id: "det-device".into(),
            entries,
            resume_token: None,
        });
        // Represent hub state as a deterministic byte string.
        let mut repr = Vec::new();
        for e in hub.entries() {
            repr.extend_from_slice(e.key.as_bytes());
            repr.extend_from_slice(&e.payload);
        }
        repr
    }

    let data = [("key/a", b"val-a" as &[u8]), ("key/b", b"val-b")];
    let state1 = build_hub(&data);
    let state2 = build_hub(&data);
    assert_eq!(state1, state2, "sync must be deterministic");
}

#[test]
fn test_changelog_replay_is_deterministic() {
    let mut journal = Journal::new("det-2");
    let s1 = journal.append("x", b"1".to_vec());
    let s2 = journal.append("y", b"2".to_vec());

    let mut log1 = ChangeLog::new();
    log1.record_put(journal.get(s1).unwrap());
    log1.record_put(journal.get(s2).unwrap());
    let result1 = log1.replay(vec![]);

    let mut log2 = ChangeLog::new();
    log2.record_put(journal.get(s1).unwrap());
    log2.record_put(journal.get(s2).unwrap());
    let result2 = log2.replay(vec![]);

    assert_eq!(result1, result2);
}

#[test]
fn test_checksum_is_deterministic() {
    let data = b"deterministic payload";
    assert_eq!(compute_checksum(data), compute_checksum(data));
}
