use crate::{AttestationLog, AttestationRecord, AttestationStatus};
#[test]
fn trusted_and_untrusted_filters() {
    let mut log = AttestationLog::new();
    log.record(AttestationRecord::new("a1", "t1", "n1", AttestationStatus::Trusted, "q", 0));
    log.record(AttestationRecord::new("a2", "t1", "n2", AttestationStatus::Untrusted, "q", 0));
    assert_eq!(log.trusted().len(), 1);
    assert_eq!(log.untrusted().len(), 1);
}
