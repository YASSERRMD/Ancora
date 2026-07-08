use crate::policy::{PolicyDecision, ThreatPolicy};
use crate::score::ThreatScore;

#[test]
fn policy_allow_zero_score() {
    let policy = ThreatPolicy::new("t1");
    let score = ThreatScore::new("i1", 0.0, 1.0);
    assert_eq!(policy.evaluate(&score), PolicyDecision::Allow);
}

#[test]
fn policy_custom_thresholds() {
    let policy = ThreatPolicy::new("t1")
        .block_threshold(50.0)
        .alert_threshold(20.0);
    let score = ThreatScore::new("i1", 55.0, 1.0);
    assert_eq!(policy.evaluate(&score), PolicyDecision::Block);
}
