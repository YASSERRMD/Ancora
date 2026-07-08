use crate::{AttestationLog, AttestationRecord, AttestationStatus};
#[test]
fn for_tenant_filters_correctly() {
    let mut log = AttestationLog::new();
    log.record(AttestationRecord::new(
        "a1",
        "t1",
        "n1",
        AttestationStatus::Trusted,
        "q",
        0,
    ));
    log.record(AttestationRecord::new(
        "a2",
        "t2",
        "n2",
        AttestationStatus::Trusted,
        "q",
        0,
    ));
    assert_eq!(log.for_tenant("t1").len(), 1);
    assert_eq!(log.for_tenant("t3").len(), 0);
}
