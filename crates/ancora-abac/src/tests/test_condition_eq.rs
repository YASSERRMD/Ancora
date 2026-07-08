use crate::{AttributeSet, AttributeValue, Condition};
fn empty() -> AttributeSet {
    AttributeSet::new()
}
#[test]
fn eq_matches() {
    let mut s = AttributeSet::new();
    s.set("dept", "eng");
    let c = Condition::Eq("dept".into(), AttributeValue::String("eng".into()));
    assert!(c.evaluate(&s, &empty(), &empty()));
}
#[test]
fn eq_fails_wrong_value() {
    let mut s = AttributeSet::new();
    s.set("dept", "hr");
    let c = Condition::Eq("dept".into(), AttributeValue::String("eng".into()));
    assert!(!c.evaluate(&s, &empty(), &empty()));
}
