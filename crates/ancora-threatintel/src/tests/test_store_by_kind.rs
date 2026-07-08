use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::store::IndicatorStore;

#[test]
fn by_kind_filter() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    ));
    s.insert(Indicator::new(
        "i2",
        "t1",
        IndicatorKind::Domain,
        "y",
        ThreatLevel::Low,
        "f",
        1,
    ));
    s.insert(Indicator::new(
        "i3",
        "t1",
        IndicatorKind::IpAddress,
        "z",
        ThreatLevel::Low,
        "f",
        1,
    ));
    assert_eq!(s.by_kind(&IndicatorKind::IpAddress).len(), 2);
    assert_eq!(s.by_kind(&IndicatorKind::Domain).len(), 1);
}
