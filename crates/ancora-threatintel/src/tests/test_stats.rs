use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::stats::ThreatIntelStats;

#[test]
fn stats_empty() {
    let s = ThreatIntelStats::for_tenant(&[], "t1");
    assert_eq!(s.total_indicators, 0);
    assert!(s.is_critical_free());
}

#[test]
fn stats_counts() {
    let i1 = Indicator::new("i1", "t1", IndicatorKind::IpAddress, "x", ThreatLevel::Critical, "f", 1);
    let i2 = Indicator::new("i2", "t1", IndicatorKind::Domain, "y", ThreatLevel::High, "f", 1);
    let v: Vec<&Indicator> = vec![&i1, &i2];
    let s = ThreatIntelStats::for_tenant(&v, "t1");
    assert_eq!(s.total_indicators, 2);
    assert_eq!(s.critical_count, 1);
    assert_eq!(s.high_count, 1);
    assert!(!s.is_critical_free());
}

#[test]
fn stats_by_kind() {
    let i1 = Indicator::new("i1", "t1", IndicatorKind::IpAddress, "x", ThreatLevel::Low, "f", 1);
    let i2 = Indicator::new("i2", "t1", IndicatorKind::IpAddress, "y", ThreatLevel::Low, "f", 1);
    let v: Vec<&Indicator> = vec![&i1, &i2];
    let s = ThreatIntelStats::for_tenant(&v, "t1");
    assert_eq!(s.by_kind.get("IP_ADDRESS").copied(), Some(2));
}
