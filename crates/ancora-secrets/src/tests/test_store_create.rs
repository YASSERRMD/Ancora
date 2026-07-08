use crate::{SecretKind, SecretStore};
#[test]
fn create_secret_succeeds_for_valid_path() {
    let mut store = SecretStore::new();
    let result = store.create(
        "t1",
        "db/password",
        SecretKind::DatabaseCredential,
        "pass",
        1,
    );
    assert!(result.is_ok());
}
#[test]
fn create_rejects_invalid_path() {
    let mut store = SecretStore::new();
    let result = store.create("t1", "db password", SecretKind::Opaque, "v", 1);
    assert!(result.is_err());
}
