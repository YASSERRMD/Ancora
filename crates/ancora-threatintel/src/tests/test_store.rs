use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::store::IndicatorStore;

#[test]
fn store_insert_get() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "1.2.3.4",
        ThreatLevel::High,
        "f",
        1,
    ));
    assert!(s.get("i1").is_some());
    assert_eq!(s.count(), 1);
}

#[test]
fn store_for_tenant() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "a.com",
        ThreatLevel::Low,
        "f",
        1,
    ));
    s.insert(Indicator::new(
        "i2",
        "t2",
        IndicatorKind::Domain,
        "b.com",
        ThreatLevel::Low,
        "f",
        1,
    ));
    assert_eq!(s.for_tenant("t1").len(), 1);
}

#[test]
fn store_active() {
    let mut s = IndicatorStore::new();
    let mut i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Url,
        "x",
        ThreatLevel::Medium,
        "f",
        1,
    );
    i.deactivate();
    s.insert(i);
    s.insert(Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Url,
        "y",
        ThreatLevel::Medium,
        "f",
        1,
    ));
    assert_eq!(s.active().len(), 1);
}

#[test]
fn store_by_value() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "1.2.3.4",
        ThreatLevel::High,
        "f",
        1,
    ));
    assert_eq!(s.by_value("1.2.3.4").len(), 1);
    assert_eq!(s.by_value("9.9.9.9").len(), 0);
}
