use crate::{NetworkPolicy, PolicyStore};
#[test]
fn policy_store_insert_and_get() {
    let mut store = PolicyStore::new();
    store.insert(NetworkPolicy::deny_by_default("t1"));
    assert!(store.get("t1").is_some());
    assert!(store.get("t999").is_none());
    assert_eq!(store.count(), 1);
}
