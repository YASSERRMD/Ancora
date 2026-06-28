use crate::{ClassificationPolicy, PolicyStore, SensitivityLevel};
#[test]
fn store_insert_and_get() {
    let mut store = PolicyStore::new();
    store.insert(ClassificationPolicy::new("t1", SensitivityLevel::Internal));
    assert!(store.get("t1").is_some());
    assert!(store.get("t999").is_none());
    assert_eq!(store.count(), 1);
}
