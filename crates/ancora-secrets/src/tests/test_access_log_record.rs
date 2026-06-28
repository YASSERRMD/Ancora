use crate::{AccessKind, AccessRecord, SecretAccessLog};
#[test]
fn access_log_records_entries() {
    let mut log = SecretAccessLog::new();
    log.record(AccessRecord::new(1, "t1", "db/pass", "alice", AccessKind::Read));
    log.record(AccessRecord::new(2, "t1", "api/key", "bob", AccessKind::Write));
    assert_eq!(log.count(), 2);
}
