use crate::revocation::{DeviceRevocationList, RevocationReason};

#[test]
fn test_compromised_device_revoked() {
    let mut drl = DeviceRevocationList::new();
    drl.revoke("compromised-device", RevocationReason::KeyCompromised, "private key leaked", 1);
    assert!(drl.is_revoked("compromised-device"));
}

#[test]
fn test_non_revoked_device() {
    let drl = DeviceRevocationList::new();
    assert!(!drl.is_revoked("clean-device"));
}

#[test]
fn test_revocation_record_stored() {
    let mut drl = DeviceRevocationList::new();
    drl.revoke("bad-device", RevocationReason::TamperDetected, "tamper event", 5);
    let record = drl.get_record("bad-device").unwrap();
    assert_eq!(record.device_id, "bad-device");
    assert_eq!(record.reason, RevocationReason::TamperDetected);
}

#[test]
fn test_revocation_count() {
    let mut drl = DeviceRevocationList::new();
    drl.revoke("dev-1", RevocationReason::Decommissioned, "retired", 10);
    drl.revoke("dev-2", RevocationReason::KeyCompromised, "leaked", 11);
    assert_eq!(drl.revoked_count(), 2);
}

#[test]
fn test_unrevoke_device() {
    let mut drl = DeviceRevocationList::new();
    drl.revoke("temp-device", RevocationReason::PolicyViolation, "policy", 20);
    assert!(drl.is_revoked("temp-device"));
    drl.unrevoke("temp-device");
    assert!(!drl.is_revoked("temp-device"));
}
