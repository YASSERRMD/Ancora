use crate::attack::{AttackOutcome, AttackStep, AttackVector};

#[test]
fn success_is_successful() {
    let s = AttackStep::new("s1", "sc1", "N", AttackVector::Network, AttackOutcome::Success, "", "", 1);
    assert!(s.is_successful());
    assert!(!s.was_detected());
    assert!(!s.was_blocked());
}

#[test]
fn partial_is_successful() {
    let s = AttackStep::new("s1", "sc1", "N", AttackVector::Local, AttackOutcome::PartialSuccess, "", "", 1);
    assert!(s.is_successful());
}

#[test]
fn detected_is_not_successful() {
    let s = AttackStep::new("s1", "sc1", "N", AttackVector::Network, AttackOutcome::Detected, "", "", 1);
    assert!(!s.is_successful());
    assert!(s.was_detected());
}

#[test]
fn blocked() {
    let s = AttackStep::new("s1", "sc1", "N", AttackVector::Network, AttackOutcome::Blocked, "", "", 1);
    assert!(!s.is_successful());
    assert!(s.was_blocked());
}
