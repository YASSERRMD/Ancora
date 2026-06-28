use crate::license::{EnterpriseCap, LicenseTier};
use crate::posture::PostureLevel;
use crate::presets::{
    community_license, default_feature_registry, enterprise_license, healthy_posture,
    standard_checkpoint,
};

#[test]
fn enterprise_license_preset() {
    let lic = enterprise_license("l1", "t1", 1);
    assert_eq!(lic.tier, LicenseTier::Enterprise);
    assert_eq!(lic.cap_count(), 10);
    assert!(lic.has_cap(&EnterpriseCap::Hsm));
    assert!(lic.has_cap(&EnterpriseCap::AirGap));
    assert!(lic.has_cap(&EnterpriseCap::ThreatIntelFeed));
    assert!(lic.is_enterprise_or_above());
}

#[test]
fn community_license_preset() {
    let lic = community_license("l2", "t1", 1);
    assert_eq!(lic.tier, LicenseTier::Community);
    assert_eq!(lic.cap_count(), 0);
    assert!(!lic.is_enterprise_or_above());
}

#[test]
fn feature_registry_preset() {
    let r = default_feature_registry();
    assert_eq!(r.count(), 7);
    assert!(r.is_enabled("hsm-integration"));
    assert!(r.is_enabled("airgap-transfer"));
    assert!(!r.is_enabled("threat-intel"));
    assert!(!r.is_enabled("quantum-safe-keys"));
}

#[test]
fn standard_checkpoint_preset() {
    let cp = standard_checkpoint(1);
    assert_eq!(cp.count(), 5);
    assert_eq!(cp.passing().len(), 4);
    assert_eq!(cp.warnings().len(), 1);
    assert_eq!(cp.failing().len(), 0);
    assert!(cp.all_healthy());
}

#[test]
fn healthy_posture_preset() {
    let p = healthy_posture("t1", 1);
    assert_eq!(p.domain_count(), 4);
    assert!(p.overall_score() >= 70);
    assert_eq!(p.posture_level(), PostureLevel::Good);
    assert_eq!(p.total_critical_findings(), 0);
}
