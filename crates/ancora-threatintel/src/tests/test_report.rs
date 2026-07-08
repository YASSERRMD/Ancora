use crate::alert::AlertStore;
use crate::audit::ThreatIntelAuditLog;
use crate::feed::FeedStore;
use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::report::ThreatIntelReport;

#[test]
fn report_empty() {
    let feeds = FeedStore::new();
    let alerts = AlertStore::new();
    let audit = ThreatIntelAuditLog::new();
    let r = ThreatIntelReport::generate(&[], &feeds, &alerts, &audit, "t1", 100);
    assert_eq!(r.total_indicators, 0);
    assert_eq!(r.open_alerts, 0);
    assert_eq!(r.tick, 100);
}

#[test]
fn report_with_data() {
    let i1 = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::High,
        "f",
        1,
    );
    let i2 = Indicator::new(
        "i2",
        "t1",
        IndicatorKind::IpAddress,
        "y",
        ThreatLevel::Medium,
        "f",
        1,
    );
    let feeds = FeedStore::new();
    let alerts = AlertStore::new();
    let audit = ThreatIntelAuditLog::new();
    let v: Vec<&Indicator> = vec![&i1, &i2];
    let r = ThreatIntelReport::generate(&v, &feeds, &alerts, &audit, "t1", 200);
    assert_eq!(r.total_indicators, 2);
    assert_eq!(r.active_indicators, 2);
}
