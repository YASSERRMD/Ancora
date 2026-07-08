use crate::{AttributeSet, AttributeValue, Condition};
fn empty() -> AttributeSet {
    AttributeSet::new()
}
#[test]
fn contains_found() {
    let mut r = AttributeSet::new();
    r.set(
        "tags",
        AttributeValue::List(vec!["sensitive".into(), "pii".into()]),
    );
    assert!(Condition::Contains("tags".into(), "pii".into()).evaluate(&empty(), &r, &empty()));
}
#[test]
fn contains_not_found() {
    let mut r = AttributeSet::new();
    r.set("tags", AttributeValue::List(vec!["public".into()]));
    assert!(!Condition::Contains("tags".into(), "secret".into()).evaluate(&empty(), &r, &empty()));
}
