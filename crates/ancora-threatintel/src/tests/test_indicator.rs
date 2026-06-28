use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};

#[test]
fn new_indicator_active() {
    let i = Indicator::new("i1", "t1", IndicatorKind::IpAddress, "1.2.3.4", ThreatLevel::High, "feed1", 100);
    assert_eq!(i.id, "i1");
    assert!(i.active);
    assert!(i.tags.is_empty());
}

#[test]
fn indicator_deactivate() {
    let mut i = Indicator::new("i1", "t1", IndicatorKind::Domain, "evil.com", ThreatLevel::Medium, "f", 1);
    i.deactivate();
    assert!(!i.active);
}

#[test]
fn indicator_expiry() {
    let i = Indicator::new("i1", "t1", IndicatorKind::Url, "http://x", ThreatLevel::Low, "f", 0)
        .with_expiry(100);
    assert!(!i.is_expired(50));
    assert!(i.is_expired(100));
    assert!(i.is_expired(200));
}

#[test]
fn indicator_with_tag_and_metadata() {
    let i = Indicator::new("i1", "t1", IndicatorKind::Email, "spam@x.com", ThreatLevel::Low, "f", 0)
        .with_tag("phishing")
        .with_metadata("campaign", "holiday");
    assert!(i.tags.contains(&"phishing".to_string()));
    assert_eq!(i.metadata.get("campaign").map(|s| s.as_str()), Some("holiday"));
}

#[test]
fn indicator_no_expiry_never_expires() {
    let i = Indicator::new("i1", "t1", IndicatorKind::FileHash, "abc123", ThreatLevel::Critical, "f", 0);
    assert!(!i.is_expired(999_999));
}
