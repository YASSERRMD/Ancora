use crate::builder::IndicatorBuilder;
use crate::indicator::{IndicatorKind, ThreatLevel};

#[test]
fn builder_defaults() {
    let i = IndicatorBuilder::new("i1", "t1", IndicatorKind::IpAddress, "1.2.3.4").build();
    assert_eq!(i.id, "i1");
    assert_eq!(i.threat_level, ThreatLevel::Medium);
    assert_eq!(i.source, "unknown");
    assert!(i.active);
}

#[test]
fn builder_custom() {
    let i = IndicatorBuilder::new("i1", "t1", IndicatorKind::Domain, "evil.com")
        .threat_level(ThreatLevel::Critical)
        .source("threat-feed")
        .tick(500)
        .tag("apt29")
        .expires_at(1000)
        .build();
    assert_eq!(i.threat_level, ThreatLevel::Critical);
    assert_eq!(i.source, "threat-feed");
    assert_eq!(i.observed_tick, 500);
    assert!(i.tags.contains(&"apt29".to_string()));
    assert_eq!(i.expires_tick, Some(1000));
}
