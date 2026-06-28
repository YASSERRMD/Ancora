use crate::SecretKind;
#[test]
fn kind_variants_are_distinct() {
    assert_ne!(SecretKind::Opaque, SecretKind::ApiKey);
    assert_ne!(SecretKind::DatabaseCredential, SecretKind::TlsCertificate);
}
