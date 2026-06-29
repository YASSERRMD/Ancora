//! Tests: idempotent sync produces no duplicates.

use crate::journal::Journal;
use crate::protocol::{Hub, SyncRequest};

#[test]
fn test_idempotent_sync_no_duplicates() {
    let mut journal = Journal::new("idem-device");
    let seq = journal.append("key/idem", b"value".to_vec());
    let entry = journal.get(seq).unwrap().clone();

    let mut hub = Hub::new();

    // Upload the same entry twice.
    let req1 = SyncRequest {
        device_id: "idem-device".into(),
        entries: vec![entry.clone()],
        resume_token: None,
    };
    let resp1 = hub.process(req1);

    let req2 = SyncRequest {
        device_id: "idem-device".into(),
        entries: vec![entry],
        resume_token: None,
    };
    let resp2 = hub.process(req2);

    // Hub should still have exactly 1 entry.
    assert_eq!(hub.len(), 1);
    // Both responses should ack the same hub seq.
    assert_eq!(resp1.acked_seqs.len(), 1);
    assert_eq!(resp2.acked_seqs.len(), 1);
    assert_eq!(resp1.acked_seqs[0], resp2.acked_seqs[0]);
}

#[test]
fn test_idempotent_sync_many_uploads() {
    let mut journal = Journal::new("idem-2");
    let seq = journal.append("k", b"v".to_vec());
    let entry = journal.get(seq).unwrap().clone();

    let mut hub = Hub::new();
    for _ in 0..5 {
        hub.process(SyncRequest {
            device_id: "idem-2".into(),
            entries: vec![entry.clone()],
            resume_token: None,
        });
    }
    assert_eq!(hub.len(), 1, "idempotent: hub must not store duplicates");
}
