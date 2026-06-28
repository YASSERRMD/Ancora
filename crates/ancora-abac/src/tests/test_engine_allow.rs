use crate::{AttributeSet, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn engine_allows_when_condition_met() {
    let mut store = PolicyStore::new();
    let mut s = AttributeSet::new(); s.set("role", "admin");
    store.add(Policy::new("p1", Effect::Allow, vec!["read".into()],
        Condition::Eq("role".into(), crate::AttributeValue::String("admin".into()))));
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("read", &s, &AttributeSet::new(), &e), Decision::Allow);
}
