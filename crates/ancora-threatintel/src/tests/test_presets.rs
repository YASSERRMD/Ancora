use crate::indicator::{IndicatorKind, ThreatLevel};
use crate::presets::{internal_feed, known_bad_ip, known_malware_hash, phishing_domain};

#[test]
fn known_bad_ip_preset() {
    let i = known_bad_ip("t1", 100);
    assert_eq!(i.kind, IndicatorKind::IpAddress);
    assert_eq!(i.threat_level, ThreatLevel::High);
    assert!(i.active);
}

#[test]
fn known_malware_hash_preset() {
    let i = known_malware_hash("t1", 100);
    assert_eq!(i.kind, IndicatorKind::FileHash);
    assert_eq!(i.threat_level, ThreatLevel::Critical);
}

#[test]
fn phishing_domain_preset() {
    let i = phishing_domain("t1", 100);
    assert_eq!(i.kind, IndicatorKind::Domain);
}

#[test]
fn internal_feed_preset() {
    let f = internal_feed("t1", 100);
    assert!(f.enabled);
    assert_eq!(f.tenant_id, "t1");
}
