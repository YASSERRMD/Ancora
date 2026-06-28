use crate::{AttestationLog, AttestationRecord, AttestationStatus};
#[test]
fn for_node_filters_correctly() {
    let mut log = AttestationLog::new();
    log.record(AttestationRecord::new("a1", "t1", "node-a", AttestationStatus::Trusted, "q", 0));
    log.record(AttestationRecord::new("a2", "t1", "node-b", AttestationStatus::Untrusted, "q", 0));
    assert_eq!(log.for_node("node-a").len(), 1);
    assert_eq!(log.for_node("node-b").len(), 1);
}
