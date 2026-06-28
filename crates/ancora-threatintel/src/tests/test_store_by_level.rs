use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::store::IndicatorStore;

#[test]
fn by_threat_level_filter() {
    let mut s = IndicatorStore::new();
    s.insert(Indicator::new("i1", "t1", IndicatorKind::Domain, "x", ThreatLevel::Critical, "f", 1));
    s.insert(Indicator::new("i2", "t1", IndicatorKind::Domain, "y", ThreatLevel::High, "f", 1));
    s.insert(Indicator::new("i3", "t1", IndicatorKind::Domain, "z", ThreatLevel::Critical, "f", 1));
    assert_eq!(s.by_threat_level(&ThreatLevel::Critical).len(), 2);
    assert_eq!(s.by_threat_level(&ThreatLevel::High).len(), 1);
    assert_eq!(s.by_threat_level(&ThreatLevel::Low).len(), 0);
}
