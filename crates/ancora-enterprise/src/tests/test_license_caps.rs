use crate::license::{EnterpriseCap, EnterpriseLicense, LicenseTier};

#[test]
fn add_caps() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1)
        .with_cap(EnterpriseCap::Hsm)
        .with_cap(EnterpriseCap::AirGap);
    assert_eq!(lic.cap_count(), 2);
    assert!(lic.has_cap(&EnterpriseCap::Hsm));
    assert!(lic.has_cap(&EnterpriseCap::AirGap));
    assert!(!lic.has_cap(&EnterpriseCap::SsoIntegration));
}

#[test]
fn no_duplicate_caps() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1)
        .with_cap(EnterpriseCap::Hsm)
        .with_cap(EnterpriseCap::Hsm);
    assert_eq!(lic.cap_count(), 1);
}
