use crate::{AttributeSet, AttributeValue, Condition};
fn empty() -> AttributeSet { AttributeSet::new() }
#[test]
fn not_eq_when_different() {
    let mut s = AttributeSet::new(); s.set("status", "active");
    let c = Condition::NotEq("status".into(), AttributeValue::String("blocked".into()));
    assert!(c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn not_eq_false_when_same() {
    let mut s = AttributeSet::new(); s.set("status", "blocked");
    let c = Condition::NotEq("status".into(), AttributeValue::String("blocked".into()));
    assert!(!c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn not_eq_missing_key_is_true() {
    let s = AttributeSet::new();
    let c = Condition::NotEq("missing".into(), AttributeValue::String("x".into()));
    assert!(c.evaluate(&s, &empty(), &empty()));
}
