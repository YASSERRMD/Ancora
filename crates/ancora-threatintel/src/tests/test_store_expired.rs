use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::store::IndicatorStore;

#[test]
fn expired_filter() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new("i1", "t1", IndicatorKind::IpAddress, "x", ThreatLevel::Low, "f", 0).with_expiry(100));
    s.insert(Indicator::new("i2", "t1", IndicatorKind::IpAddress, "y", ThreatLevel::Low, "f", 0).with_expiry(500));
    s.insert(Indicator::new("i3", "t1", IndicatorKind::IpAddress, "z", ThreatLevel::Low, "f", 0));
    assert_eq!(s.expired(200).len(), 1);
    assert_eq!(s.expired(600).len(), 2);
    assert_eq!(s.expired(50).len(), 0);
}
