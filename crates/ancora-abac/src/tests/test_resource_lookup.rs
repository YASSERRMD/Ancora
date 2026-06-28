use crate::{AttributeSet, AttributeValue, Condition};
#[test]
fn condition_finds_attr_in_resource() {
    let s = AttributeSet::new();
    let mut r = AttributeSet::new(); r.set("owner", "alice");
    let e = AttributeSet::new();
    let c = Condition::Eq("owner".into(), AttributeValue::String("alice".into()));
    assert!(c.evaluate(&s, &r, &e));
}
#[test]
fn subject_attr_shadows_resource_attr() {
    let mut s = AttributeSet::new(); s.set("role", "admin");
    let mut r = AttributeSet::new(); r.set("role", "viewer");
    let e = AttributeSet::new();
    let c = Condition::Eq("role".into(), AttributeValue::String("admin".into()));
    assert!(c.evaluate(&s, &r, &e));
}
