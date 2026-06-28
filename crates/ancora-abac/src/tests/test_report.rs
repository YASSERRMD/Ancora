use crate::report::report;
use crate::{Condition, Effect, Policy, PolicyStore};
#[test]
fn report_counts_correct() {
    let mut store = PolicyStore::new();
    store.add(Policy::new("a1", Effect::Allow, vec!["read".into()], Condition::Exists("x".into())));
    store.add(Policy::new("d1", Effect::Deny, vec!["*".into()], Condition::Exists("y".into())));
    let r = report(&store);
    assert_eq!(r.total, 2);
    assert_eq!(r.allow_count, 1);
    assert_eq!(r.deny_count, 1);
    assert_eq!(r.wildcard_count, 1);
}
