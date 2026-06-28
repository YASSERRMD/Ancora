use crate::license::{EnterpriseLicense, LicenseTier};

#[test]
fn basic_fields() {
    let lic = EnterpriseLicense::new("lic-1", "t1", LicenseTier::Enterprise, 500, 5, 100);
    assert_eq!(lic.id, "lic-1");
    assert_eq!(lic.tenant_id, "t1");
    assert_eq!(lic.tier, LicenseTier::Enterprise);
    assert_eq!(lic.max_users, 500);
    assert_eq!(lic.max_tenants, 5);
    assert_eq!(lic.issued_tick, 100);
    assert!(lic.expires_tick.is_none());
    assert_eq!(lic.cap_count(), 0);
}

#[test]
fn enterprise_or_above_enterprise() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Enterprise, 100, 1, 1);
    assert!(lic.is_enterprise_or_above());
}

#[test]
fn enterprise_or_above_gov() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::GovCloud, 100, 1, 1);
    assert!(lic.is_enterprise_or_above());
}

#[test]
fn community_not_enterprise() {
    let lic = EnterpriseLicense::new("l1", "t1", LicenseTier::Community, 10, 1, 1);
    assert!(!lic.is_enterprise_or_above());
}
