use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::stats::ThreatIntelStats;

#[test]
fn critical_free_when_no_critical() {
    let i = Indicator::new("i1", "t1", IndicatorKind::Domain, "x", ThreatLevel::High, "f", 1);
    let v: Vec<&Indicator> = vec![&i];
    let s = ThreatIntelStats::for_tenant(&v, "t1");
    assert!(s.is_critical_free());
}

#[test]
fn not_critical_free_with_critical() {
    let i = Indicator::new("i1", "t1", IndicatorKind::IpAddress, "x", ThreatLevel::Critical, "f", 1);
    let v: Vec<&Indicator> = vec![&i];
    let s = ThreatIntelStats::for_tenant(&v, "t1");
    assert!(!s.is_critical_free());
}
