use crate::indicator::ThreatLevel;

#[test]
fn threat_level_order() {
    assert!(ThreatLevel::Informational < ThreatLevel::Low);
    assert!(ThreatLevel::Low < ThreatLevel::Medium);
    assert!(ThreatLevel::Medium < ThreatLevel::High);
    assert!(ThreatLevel::High < ThreatLevel::Critical);
}
