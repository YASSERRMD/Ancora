use crate::{AttributeSet, AttributeValue, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn deny_policy_overrides_allow() {
    let mut store = PolicyStore::new();
    store.add(Policy::new("allow-all", Effect::Allow, vec!["*".into()], Condition::Exists("user".into())).with_priority(20));
    store.add(Policy::new("deny-blocked", Effect::Deny, vec!["*".into()],
        Condition::Eq("blocked".into(), AttributeValue::Bool(true))).with_priority(10));
    let mut s = AttributeSet::new(); s.set("user", "alice"); s.set("blocked", true);
    let engine = PolicyEngine::new(&store);
    let empty = AttributeSet::new();
    assert!(matches!(engine.evaluate("read", &s, &empty, &empty), Decision::Deny(_)));
}
