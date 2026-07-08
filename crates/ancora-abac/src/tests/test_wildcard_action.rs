use crate::{AttributeSet, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn wildcard_policy_applies_to_all_actions() {
    let mut store = PolicyStore::new();
    store.add(Policy::new(
        "allow-all",
        Effect::Allow,
        vec!["*".into()],
        Condition::Exists("user".into()),
    ));
    let mut s = AttributeSet::new();
    s.set("user", "bob");
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    for action in ["read", "write", "delete", "execute"] {
        assert_eq!(
            engine.evaluate(action, &s, &e, &e),
            Decision::Allow,
            "failed for {action}"
        );
    }
}
