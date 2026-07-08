use crate::indicator::ThreatLevel;
use crate::score::ThreatScore;

#[test]
fn score_level_mapping() {
    assert_eq!(
        ThreatScore::new("x", 95.0, 1.0).level,
        ThreatLevel::Critical
    );
    assert_eq!(ThreatScore::new("x", 75.0, 1.0).level, ThreatLevel::High);
    assert_eq!(ThreatScore::new("x", 50.0, 1.0).level, ThreatLevel::Medium);
    assert_eq!(ThreatScore::new("x", 15.0, 1.0).level, ThreatLevel::Low);
    assert_eq!(
        ThreatScore::new("x", 5.0, 1.0).level,
        ThreatLevel::Informational
    );
}
