use crate::{AccessKind, AccessRecord, SecretAccessLog};
#[test]
fn reads_for_filters_by_path_and_tenant() {
    let mut log = SecretAccessLog::new();
    log.record(AccessRecord::new(
        1,
        "t1",
        "db/pass",
        "alice",
        AccessKind::Read,
    ));
    log.record(AccessRecord::new(
        2,
        "t1",
        "api/key",
        "bob",
        AccessKind::Read,
    ));
    log.record(AccessRecord::new(
        3,
        "t1",
        "db/pass",
        "carol",
        AccessKind::Write,
    ));
    let reads = log.reads_for("t1", "db/pass");
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].subject, "alice");
}
