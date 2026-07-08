use crate::{Secret, SecretKind, SecretStatus};
#[test]
fn secret_display_contains_path_and_tenant() {
    let s = Secret::new(
        "db/password",
        "acme",
        SecretKind::DatabaseCredential,
        "val",
        1,
    );
    let output = format!("{}", s);
    assert!(output.contains("db/password"));
    assert!(output.contains("acme"));
}
#[test]
fn secret_status_display_active() {
    assert_eq!(format!("{}", SecretStatus::Active), "active");
}
#[test]
fn secret_status_display_rotated() {
    assert_eq!(format!("{}", SecretStatus::Rotated), "rotated");
}
