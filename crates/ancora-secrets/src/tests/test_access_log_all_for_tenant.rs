use crate::{AccessKind, AccessRecord, SecretAccessLog};
#[test]
fn all_for_tenant_includes_all_paths_for_that_tenant() {
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
        "t2",
        "db/pass",
        "bob",
        AccessKind::Read,
    ));
    log.record(AccessRecord::new(
        3,
        "t1",
        "api/key",
        "alice",
        AccessKind::Write,
    ));
    let t1 = log.all_for_tenant("t1");
    assert_eq!(t1.len(), 2);
}
