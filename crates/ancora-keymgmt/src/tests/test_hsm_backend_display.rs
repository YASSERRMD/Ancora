use crate::HsmBackend;
#[test]
fn hsm_backend_display() {
    assert_eq!(format!("{}", HsmBackend::Software), "SOFTWARE");
    assert_eq!(format!("{}", HsmBackend::CloudKms), "CLOUD_KMS");
    assert_eq!(format!("{}", HsmBackend::Pkcs11), "PKCS11");
}
