use crate::{SecretKind, SecretStore};
#[test]
fn create_duplicate_path_returns_error() {
    let mut store = SecretStore::new();
    store.create("t1", "db/pass", SecretKind::Opaque, "v1", 1).unwrap();
    let result = store.create("t1", "db/pass", SecretKind::Opaque, "v2", 2);
    assert!(result.is_err());
}
