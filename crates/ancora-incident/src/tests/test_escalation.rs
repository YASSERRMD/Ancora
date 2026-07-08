use crate::escalation::{EscalationChannel, EscalationLevel, EscalationPolicy};
use crate::incident::Severity;

#[test]
fn policy_should_escalate() {
    let policy = EscalationPolicy::new("t1", Severity::High);
    assert!(policy.should_escalate(&Severity::High));
    assert!(policy.should_escalate(&Severity::Critical));
    assert!(!policy.should_escalate(&Severity::Low));
    assert!(!policy.should_escalate(&Severity::Medium));
}

#[test]
fn policy_add_levels() {
    let mut policy = EscalationPolicy::new("t1", Severity::Medium);
    policy.add_level(EscalationLevel::new(
        1,
        "alice",
        EscalationChannel::Pager,
        0,
    ));
    policy.add_level(EscalationLevel::new(
        2,
        "bob",
        EscalationChannel::Phone,
        300,
    ));
    assert_eq!(policy.level_count(), 2);
    assert_eq!(
        policy.level_at(0).map(|l| l.on_call.as_str()),
        Some("alice")
    );
    assert_eq!(policy.level_at(1).map(|l| l.delay_ticks), Some(300));
}

#[test]
fn policy_level_at_out_of_bounds() {
    let policy = EscalationPolicy::new("t1", Severity::Low);
    assert!(policy.level_at(99).is_none());
}
