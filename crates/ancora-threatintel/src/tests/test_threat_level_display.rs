use crate::indicator::ThreatLevel;

#[test]
fn threat_level_display() {
    assert_eq!(format!("{}", ThreatLevel::Informational), "INFORMATIONAL");
    assert_eq!(format!("{}", ThreatLevel::Low), "LOW");
    assert_eq!(format!("{}", ThreatLevel::Medium), "MEDIUM");
    assert_eq!(format!("{}", ThreatLevel::High), "HIGH");
    assert_eq!(format!("{}", ThreatLevel::Critical), "CRITICAL");
}
