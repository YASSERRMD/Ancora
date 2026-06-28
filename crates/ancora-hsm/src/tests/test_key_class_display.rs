use crate::key::KeyClass;

#[test]
fn display_secret_key() {
    assert_eq!(format!("{}", KeyClass::SecretKey), "SECRET");
}

#[test]
fn display_public_key() {
    assert_eq!(format!("{}", KeyClass::PublicKey), "PUBLIC");
}

#[test]
fn display_private_key() {
    assert_eq!(format!("{}", KeyClass::PrivateKey), "PRIVATE");
}
