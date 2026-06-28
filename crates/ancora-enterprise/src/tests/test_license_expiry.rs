use crate::license::{EnterpriseLicense, LicenseTier};

#[test]
fn no_expiry_always_valid() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1);
    assert!(lic.is_valid(9999));
    assert!(!lic.is_expired(9999));
}

#[test]
fn expired_after_tick() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1)
        .with_expiry(100);
    assert!(!lic.is_expired(100));
    assert!(lic.is_expired(101));
    assert!(!lic.is_valid(101));
}
