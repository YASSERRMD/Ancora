use crate::{Condition, Effect, Policy};
#[test]
fn policy_matches_specific_action() {
    let p = Policy::new("p1", Effect::Allow, vec!["read".into()], Condition::Exists("x".into()));
    assert!(p.applies_to_action("read"));
    assert!(!p.applies_to_action("write"));
}
#[test]
fn policy_wildcard_matches_all() {
    let p = Policy::new("p2", Effect::Allow, vec!["*".into()], Condition::Exists("x".into()));
    assert!(p.applies_to_action("read"));
    assert!(p.applies_to_action("delete"));
}
