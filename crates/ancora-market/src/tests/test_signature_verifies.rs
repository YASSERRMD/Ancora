use crate::identity::{sign_payload, verify_signature, AuthorIdentity, IdentityError};

#[test]
fn verified_author_signature_accepted() {
    let mut id = AuthorIdentity::new("carol", "Carol", "PUBKEY_CAROL").unwrap();
    id.mark_verified();
    let payload = b"extension-bundle-v1";
    let sig = sign_payload(&id, payload);
    assert!(verify_signature(&id, payload, &sig).is_ok());
}

#[test]
fn unverified_author_rejected() {
    let id = AuthorIdentity::new("dave", "Dave", "PUBKEY_DAVE").unwrap();
    let payload = b"some-payload";
    let sig = sign_payload(&id, payload);
    assert_eq!(
        verify_signature(&id, payload, &sig),
        Err(IdentityError::UnverifiedAuthor)
    );
}

#[test]
fn tampered_payload_fails_verification() {
    let mut id = AuthorIdentity::new("eve", "Eve", "PUBKEY_EVE").unwrap();
    id.mark_verified();
    // Sign a short payload; verify against a longer tampered payload
    let sig = sign_payload(&id, b"hello");
    assert_eq!(
        verify_signature(&id, b"tampered-payload-is-longer", &sig),
        Err(IdentityError::SignatureMismatch)
    );
}

#[test]
fn wrong_signer_handle_rejected() {
    let mut alice = AuthorIdentity::new("alice", "Alice", "PUBKEY_A").unwrap();
    alice.mark_verified();
    let mut mallory = AuthorIdentity::new("mallory", "Mallory", "PUBKEY_M").unwrap();
    mallory.mark_verified();
    let payload = b"legitimate-payload";
    let alice_sig = sign_payload(&alice, payload);
    // mallory tries to claim alice's signature
    assert!(matches!(
        verify_signature(&mallory, payload, &alice_sig),
        Err(IdentityError::UnknownSigner(_))
    ));
}
