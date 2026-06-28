use crate::{AttestationRecord, AttestationStatus};
#[test]
fn trusted_record_is_trusted() {
    let r = AttestationRecord::new("a1", "t1", "node1", AttestationStatus::Trusted, "quote", 10);
    assert!(r.is_trusted());
    assert_eq!(format!("{}", r.status), "TRUSTED");
}
#[test]
fn untrusted_record_is_not_trusted() {
    let r = AttestationRecord::new("a2", "t1", "node1", AttestationStatus::Untrusted, "bad", 10);
    assert!(!r.is_trusted());
}
