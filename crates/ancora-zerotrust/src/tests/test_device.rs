use crate::device::{DevicePosture, TrustLevel};

#[test]
fn new_device_untrusted() {
    let d = DevicePosture::new("d1", "t1", 1);
    assert_eq!(d.trust_level, TrustLevel::Untrusted);
    assert!(!d.is_trusted());
}

#[test]
fn compute_trust_fully_trusted() {
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.disk_encrypted = true;
    d.compute_trust();
    assert_eq!(d.trust_level, TrustLevel::FullyTrusted);
    assert!(d.is_trusted());
}

#[test]
fn compute_trust_partial() {
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.compute_trust();
    assert_eq!(d.trust_level, TrustLevel::Partial);
    assert!(!d.is_trusted());
}

#[test]
fn compute_trust_trusted_two_checks() {
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.compute_trust();
    assert_eq!(d.trust_level, TrustLevel::Trusted);
    assert!(d.is_trusted());
}
