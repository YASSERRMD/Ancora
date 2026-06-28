use crate::{BootPolicy, PolicyStore};
#[test]
fn policy_store_insert_get_remove() {
    let mut store = PolicyStore::new();
    store.insert(BootPolicy::new("t1"));
    assert!(store.get("t1").is_some());
    assert_eq!(store.count(), 1);
    store.remove("t1");
    assert!(store.get("t1").is_none());
}
