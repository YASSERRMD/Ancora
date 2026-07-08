use crate::escalation::{default_policy_for, EscalationPolicy};
use crate::incident::Severity;

#[test]
fn p1_policy_has_three_tiers() {
    let p = default_policy_for(&Severity::P1);
    assert_eq!(p.tier_count(), 3);
}

#[test]
fn p3_policy_has_one_tier() {
    let p = default_policy_for(&Severity::P3);
    assert_eq!(p.tier_count(), 1);
}

#[test]
fn tier_at_zero_is_first_tier() {
    let policy = EscalationPolicy::new("p")
        .add_tier("primary", 0)
        .add_tier("secondary", 300);
    let t = policy.tier_at_elapsed(0).unwrap();
    assert_eq!(t.contact, "primary");
}

#[test]
fn tier_escalates_after_wait() {
    let policy = EscalationPolicy::new("p")
        .add_tier("primary", 0)
        .add_tier("secondary", 300);
    let t = policy.tier_at_elapsed(300).unwrap();
    assert_eq!(t.contact, "secondary");
}
