use crate::{Secret, SecretKind};
#[test]
fn new_secret_has_version_one() {
    let s = Secret::new(
        "db/password",
        "t1",
        SecretKind::DatabaseCredential,
        "s3cr3t",
        1,
    );
    assert_eq!(s.active_version, 1);
    assert_eq!(s.version_count(), 1);
}
#[test]
fn new_secret_path_and_tenant_stored() {
    let s = Secret::new("api/key", "acme", SecretKind::ApiKey, "val", 10);
    assert_eq!(s.path, "api/key");
    assert_eq!(s.tenant_id, "acme");
}
