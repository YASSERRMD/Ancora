use crate::license::LicenseTier;

#[test]
fn community() {
    assert_eq!(LicenseTier::Community.to_string(), "COMMUNITY");
}
#[test]
fn professional() {
    assert_eq!(LicenseTier::Professional.to_string(), "PROFESSIONAL");
}
#[test]
fn enterprise() {
    assert_eq!(LicenseTier::Enterprise.to_string(), "ENTERPRISE");
}
#[test]
fn gov_cloud() {
    assert_eq!(LicenseTier::GovCloud.to_string(), "GOV_CLOUD");
}
