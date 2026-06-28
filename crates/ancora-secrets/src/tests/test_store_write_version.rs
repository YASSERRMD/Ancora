use crate::{SecretKind, SecretStore};
#[test]
fn write_version_changes_active_value() {
    let mut store = SecretStore::new();
    store.create("t1", "api/key", SecretKind::ApiKey, "old-key", 1).unwrap();
    store.write_version("t1", "api/key", "new-key", 2).unwrap();
    let s = store.read("t1", "api/key").unwrap();
    assert_eq!(s.active_value(), Some("new-key"));
}
