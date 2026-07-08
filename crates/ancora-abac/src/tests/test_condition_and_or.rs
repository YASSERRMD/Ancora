use crate::{AttributeSet, Condition};
fn empty() -> AttributeSet {
    AttributeSet::new()
}
#[test]
fn and_both_true() {
    let mut s = AttributeSet::new();
    s.set("a", 1i64);
    s.set("b", 2i64);
    let c = Condition::And(
        Box::new(Condition::GreaterThan("a".into(), 0)),
        Box::new(Condition::GreaterThan("b".into(), 1)),
    );
    assert!(c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn or_one_true() {
    let mut s = AttributeSet::new();
    s.set("a", 0i64);
    s.set("b", 5i64);
    let c = Condition::Or(
        Box::new(Condition::GreaterThan("a".into(), 1)),
        Box::new(Condition::GreaterThan("b".into(), 1)),
    );
    assert!(c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn and_one_false_is_false() {
    let mut s = AttributeSet::new();
    s.set("a", 0i64);
    let c = Condition::And(
        Box::new(Condition::GreaterThan("a".into(), 1)),
        Box::new(Condition::Exists("a".into())),
    );
    assert!(!c.evaluate(&s, &empty(), &empty()));
}
