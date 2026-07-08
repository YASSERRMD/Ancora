use crate::alert::AlertStore;
use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::summary::ThreatIntelSummary;

#[test]
fn summary_healthy() {
    let alerts = AlertStore::new();
    let s = ThreatIntelSummary::generate(&[], &alerts, "t1");
    assert!(s.is_healthy);
    assert_eq!(s.total_indicators, 0);
}

#[test]
fn summary_unhealthy_critical() {
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::Critical,
        "f",
        1,
    );
    let alerts = AlertStore::new();
    let v: Vec<&Indicator> = vec![&i];
    let s = ThreatIntelSummary::generate(&v, &alerts, "t1");
    assert!(!s.is_healthy);
    assert_eq!(s.critical_count, 1);
}
