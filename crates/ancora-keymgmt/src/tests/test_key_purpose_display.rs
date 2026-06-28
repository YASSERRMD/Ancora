use crate::KeyPurpose;
#[test]
fn purpose_display() {
    assert_eq!(format!("{}", KeyPurpose::Encryption), "ENCRYPTION");
    assert_eq!(format!("{}", KeyPurpose::Signing), "SIGNING");
    assert_eq!(format!("{}", KeyPurpose::KeyWrapping), "KEY_WRAPPING");
}
