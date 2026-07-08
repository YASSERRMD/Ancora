use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};
use crate::policy::{PolicyDecision, ThreatPolicy};
use crate::score::ThreatScore;

#[test]
fn policy_block_high_score() {
    let policy = ThreatPolicy::new("t1");
    let score = ThreatScore::new("i1", 80.0, 0.9);
    assert_eq!(policy.evaluate(&score), PolicyDecision::Block);
}

#[test]
fn policy_alert_medium_score() {
    let policy = ThreatPolicy::new("t1");
    let score = ThreatScore::new("i1", 55.0, 0.8);
    assert_eq!(policy.evaluate(&score), PolicyDecision::Alert);
}

#[test]
fn policy_monitor_low_confidence() {
    let policy = ThreatPolicy::new("t1");
    let score = ThreatScore::new("i1", 80.0, 0.3);
    assert_eq!(policy.evaluate(&score), PolicyDecision::Monitor);
}

#[test]
fn policy_should_block_high_indicator() {
    let policy = ThreatPolicy::new("t1");
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::IpAddress,
        "x",
        ThreatLevel::High,
        "f",
        1,
    );
    assert!(policy.should_block_indicator(&i));
}

#[test]
fn policy_no_block_low_indicator() {
    let policy = ThreatPolicy::new("t1");
    let i = Indicator::new(
        "i1",
        "t1",
        IndicatorKind::Domain,
        "x",
        ThreatLevel::Low,
        "f",
        1,
    );
    assert!(!policy.should_block_indicator(&i));
}
