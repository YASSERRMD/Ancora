use crate::{AttributeSet, AttributeValue, Condition};
fn empty() -> AttributeSet { AttributeSet::new() }
#[test]
fn nested_and_or_condition() {
    let mut s = AttributeSet::new(); s.set("dept", "eng"); s.set("level", 5i64);
    let cond = Condition::And(
        Box::new(Condition::Eq("dept".into(), AttributeValue::String("eng".into()))),
        Box::new(Condition::Or(
            Box::new(Condition::GreaterThan("level".into(), 3)),
            Box::new(Condition::Exists("admin".into())),
        )),
    );
    assert!(cond.evaluate(&s, &empty(), &empty()));
}
