use crate::{SecretKind, SecretStore};
#[test]
fn store_count_reflects_all_tenants() {
    let mut store = SecretStore::new();
    store.create("t1", "a/b", SecretKind::Opaque, "v", 1).unwrap();
    store.create("t2", "a/b", SecretKind::Opaque, "v", 2).unwrap();
    assert_eq!(store.count(), 2);
}
