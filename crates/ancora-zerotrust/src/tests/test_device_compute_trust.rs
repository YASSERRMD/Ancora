use crate::device::{DevicePosture, TrustLevel};

#[test]
fn untrusted_with_no_checks() {
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.compute_trust();
    assert_eq!(d.trust_level, TrustLevel::Untrusted);
}

#[test]
fn fully_trusted_all_checks() {
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.disk_encrypted = true;
    d.compute_trust();
    assert!(d.is_trusted());
}
