use crate::{AttributeSet, Condition, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn is_allowed_true_for_allow() {
    let mut store = PolicyStore::new();
    store.add(Policy::new("p", Effect::Allow, vec!["read".into()], Condition::Exists("user".into())));
    let mut s = AttributeSet::new(); s.set("user", "x");
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert!(engine.is_allowed("read", &s, &e, &e));
}
#[test]
fn is_allowed_false_for_not_applicable() {
    let store = PolicyStore::new();
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert!(!engine.is_allowed("read", &e, &e, &e));
}
