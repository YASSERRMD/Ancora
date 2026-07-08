use crate::{AttributeSet, Condition};
fn empty() -> AttributeSet {
    AttributeSet::new()
}
#[test]
fn greater_than_true() {
    let mut s = AttributeSet::new();
    s.set("level", 5i64);
    assert!(Condition::GreaterThan("level".into(), 3).evaluate(&s, &empty(), &empty()));
}
#[test]
fn less_than_true() {
    let mut s = AttributeSet::new();
    s.set("age", 17i64);
    assert!(Condition::LessThan("age".into(), 18).evaluate(&s, &empty(), &empty()));
}
#[test]
fn gt_false_when_equal() {
    let mut s = AttributeSet::new();
    s.set("v", 5i64);
    assert!(!Condition::GreaterThan("v".into(), 5).evaluate(&s, &empty(), &empty()));
}
