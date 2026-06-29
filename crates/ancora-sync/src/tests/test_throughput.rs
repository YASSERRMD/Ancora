//! Tests: sync throughput measurement.

use crate::journal::Journal;
use crate::protocol::{Hub, SyncRequest};
use std::time::Instant;

#[test]
fn test_sync_throughput_measured() {
    const ENTRY_COUNT: usize = 1_000;

    let mut journal = Journal::new("throughput-device");
    for i in 0..ENTRY_COUNT {
        journal.append(format!("key/{i}"), format!("value-{i}").into_bytes());
    }
    let entries = journal.all_entries().to_vec();

    let mut hub = Hub::new();
    let start = Instant::now();

    hub.process(SyncRequest {
        device_id: "throughput-device".into(),
        entries,
        resume_token: None,
    });

    let elapsed = start.elapsed();
    assert_eq!(hub.len(), ENTRY_COUNT);

    // We just assert it completes in a reasonable time (<5s), not a strict
    // performance SLA.  The test proves the path is exercised.
    assert!(
        elapsed.as_secs() < 5,
        "sync of {ENTRY_COUNT} entries took {elapsed:?}, expected <5s"
    );
}
