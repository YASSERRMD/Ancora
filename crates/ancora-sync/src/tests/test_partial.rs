//! Tests: partial sync resumes correctly.

use crate::journal::Journal;
use crate::partial::PartialSyncSession;
use crate::protocol::Hub;

#[test]
fn test_partial_sync_resumes() {
    let mut journal = Journal::new("edge-1");
    for i in 0..10u64 {
        journal.append(format!("key/{i}"), vec![i as u8]);
    }
    let entries: Vec<_> = journal.all_entries().to_vec();

    let mut hub = Hub::new();
    let mut session = PartialSyncSession::new("edge-1", entries, 3);

    // First step uploads 3 entries.
    let resp = session.step(&mut hub);
    assert_eq!(resp.acked_seqs.len(), 3);
    assert_eq!(session.remaining(), 7);

    // Continue to completion.
    session.run_to_completion(&mut hub);
    assert!(session.is_complete());
    assert_eq!(hub.len(), 10);
}

#[test]
fn test_partial_sync_single_batch() {
    let mut journal = Journal::new("edge-2");
    journal.append("k", b"v".to_vec());
    let entries = journal.all_entries().to_vec();

    let mut hub = Hub::new();
    let mut session = PartialSyncSession::new("edge-2", entries, 100);
    session.run_to_completion(&mut hub);

    assert!(session.is_complete());
    assert_eq!(session.acked_seqs().len(), 1);
}

#[test]
fn test_partial_sync_empty() {
    let mut hub = Hub::new();
    let mut session = PartialSyncSession::new("edge-3", vec![], 5);
    assert!(session.is_complete());
    session.run_to_completion(&mut hub);
    assert_eq!(hub.len(), 0);
}
