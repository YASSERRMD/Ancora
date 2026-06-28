use crate::{AttributeSet, Condition};
fn empty() -> AttributeSet { AttributeSet::new() }
#[test]
fn exists_true_when_present() {
    let mut s = AttributeSet::new(); s.set("key", "val");
    assert!(Condition::Exists("key".into()).evaluate(&s, &empty(), &empty()));
}
#[test]
fn not_exists_when_missing() {
    let s = AttributeSet::new();
    assert!(Condition::NotExists("missing".into()).evaluate(&s, &empty(), &empty()));
}
