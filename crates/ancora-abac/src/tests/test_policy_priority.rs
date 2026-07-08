use crate::{Condition, Effect, Policy, PolicyStore};
#[test]
fn policies_sorted_by_priority() {
    let mut store = PolicyStore::new();
    store.add(
        Policy::new(
            "high",
            Effect::Deny,
            vec!["*".into()],
            Condition::Exists("x".into()),
        )
        .with_priority(200),
    );
    store.add(
        Policy::new(
            "low",
            Effect::Allow,
            vec!["*".into()],
            Condition::Exists("x".into()),
        )
        .with_priority(50),
    );
    assert_eq!(store.policies()[0].id, "low");
    assert_eq!(store.policies()[1].id, "high");
}
