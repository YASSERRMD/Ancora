use crate::audit::HsmOperation;

#[test]
fn display_generate_key() {
    assert_eq!(format!("{}", HsmOperation::GenerateKey), "GENERATE_KEY");
}

#[test]
fn display_delete_key() {
    assert_eq!(format!("{}", HsmOperation::DeleteKey), "DELETE_KEY");
}

#[test]
fn display_sign() {
    assert_eq!(format!("{}", HsmOperation::Sign), "SIGN");
}

#[test]
fn display_verify() {
    assert_eq!(format!("{}", HsmOperation::Verify), "VERIFY");
}

#[test]
fn display_encrypt() {
    assert_eq!(format!("{}", HsmOperation::Encrypt), "ENCRYPT");
}

#[test]
fn display_decrypt() {
    assert_eq!(format!("{}", HsmOperation::Decrypt), "DECRYPT");
}

#[test]
fn display_wrap_key() {
    assert_eq!(format!("{}", HsmOperation::WrapKey), "WRAP_KEY");
}

#[test]
fn display_unwrap_key() {
    assert_eq!(format!("{}", HsmOperation::UnwrapKey), "UNWRAP_KEY");
}

#[test]
fn display_session_opened() {
    assert_eq!(format!("{}", HsmOperation::SessionOpened), "SESSION_OPENED");
}

#[test]
fn display_session_closed() {
    assert_eq!(format!("{}", HsmOperation::SessionClosed), "SESSION_CLOSED");
}
