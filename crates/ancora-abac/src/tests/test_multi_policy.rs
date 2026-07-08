use crate::{AttributeSet, Condition, Decision, Effect, Policy, PolicyEngine, PolicyStore};
#[test]
fn first_matching_allow_wins() {
    let mut store = PolicyStore::new();
    store.add(
        Policy::new(
            "p1",
            Effect::Allow,
            vec!["read".into()],
            Condition::Exists("user".into()),
        )
        .with_priority(10),
    );
    store.add(
        Policy::new(
            "p2",
            Effect::Allow,
            vec!["read".into()],
            Condition::Exists("user".into()),
        )
        .with_priority(20),
    );
    let mut s = AttributeSet::new();
    s.set("user", "alice");
    let engine = PolicyEngine::new(&store);
    let e = AttributeSet::new();
    assert_eq!(engine.evaluate("read", &s, &e, &e), Decision::Allow);
}
#[test]
fn policy_count_correct() {
    let mut store = PolicyStore::new();
    store.add(Policy::new(
        "p1",
        Effect::Allow,
        vec!["*".into()],
        Condition::Exists("x".into()),
    ));
    store.add(Policy::new(
        "p2",
        Effect::Deny,
        vec!["*".into()],
        Condition::Exists("y".into()),
    ));
    assert_eq!(store.count(), 2);
}
