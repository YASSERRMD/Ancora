use crate::{AttributeSet, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn engine_denies_when_deny_policy_matches() {
    let mut store = PolicyStore::new();
    let mut s = AttributeSet::new();
    s.set("blocked", true);
    store.add(Policy::new(
        "deny-blocked",
        Effect::Deny,
        vec!["*".into()],
        Condition::Exists("blocked".into()),
    ));
    let engine = PolicyEngine::new(&store);
    let empty = AttributeSet::new();
    assert!(matches!(
        engine.evaluate("read", &s, &empty, &empty),
        Decision::Deny(_)
    ));
}
