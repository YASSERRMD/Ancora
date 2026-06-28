use crate::{AttributeSet, AttributeValue, Condition};
fn empty() -> AttributeSet { AttributeSet::new() }
#[test]
fn not_flips_true_to_false() {
    let mut s = AttributeSet::new(); s.set("x", "y");
    let c = Condition::Not(Box::new(Condition::Eq("x".into(), AttributeValue::String("y".into()))));
    assert!(!c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn not_flips_false_to_true() {
    let s = AttributeSet::new();
    let c = Condition::Not(Box::new(Condition::Exists("missing".into())));
    assert!(c.evaluate(&s, &empty(), &empty()));
}
