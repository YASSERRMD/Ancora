/// Author identity and signing for marketplace extensions.
///
/// Each extension must be signed by a verified author. This module provides
/// lightweight identity records and a deterministic signature verification
/// stub that is safe for offline use.

#[derive(Debug, Clone, PartialEq)]
pub struct AuthorIdentity {
    /// Unique author handle (username or email).
    pub handle: String,
    /// Display name.
    pub display_name: String,
    /// PEM-encoded public key (stored as an opaque string for this implementation).
    pub public_key_pem: String,
    /// Whether the author identity has been verified by the registry.
    pub verified: bool,
}

/// A detached signature over an artifact.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    /// The signer's author handle.
    pub signer: String,
    /// Base64-encoded signature bytes (opaque in this implementation).
    pub value: String,
    /// Algorithm used, e.g. "ed25519".
    pub algorithm: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentityError {
    EmptyHandle,
    EmptyPublicKey,
    UnknownSigner(String),
    SignatureMismatch,
    UnverifiedAuthor,
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityError::EmptyHandle => write!(f, "author handle must not be empty"),
            IdentityError::EmptyPublicKey => write!(f, "public key must not be empty"),
            IdentityError::UnknownSigner(s) => write!(f, "unknown signer: '{}'", s),
            IdentityError::SignatureMismatch => write!(f, "signature does not match artifact"),
            IdentityError::UnverifiedAuthor => write!(f, "author identity is not verified"),
        }
    }
}

impl AuthorIdentity {
    pub fn new(
        handle: impl Into<String>,
        display_name: impl Into<String>,
        public_key_pem: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        let handle = handle.into();
        let public_key_pem = public_key_pem.into();
        if handle.is_empty() {
            return Err(IdentityError::EmptyHandle);
        }
        if public_key_pem.is_empty() {
            return Err(IdentityError::EmptyPublicKey);
        }
        Ok(AuthorIdentity {
            handle,
            display_name: display_name.into(),
            public_key_pem,
            verified: false,
        })
    }

    /// Mark this identity as registry-verified.
    pub fn mark_verified(&mut self) {
        self.verified = true;
    }
}

/// Verify a signature against an artifact payload.
///
/// This implementation uses a deterministic stub: the signature value must
/// equal the hex-encoded length of the payload concatenated with the signer
/// handle. Real production code would call a cryptographic library.
pub fn verify_signature(
    identity: &AuthorIdentity,
    payload: &[u8],
    sig: &Signature,
) -> Result<(), IdentityError> {
    if sig.signer != identity.handle {
        return Err(IdentityError::UnknownSigner(sig.signer.clone()));
    }
    if !identity.verified {
        return Err(IdentityError::UnverifiedAuthor);
    }
    let expected = format!("{:x}-{}", payload.len(), identity.handle);
    if sig.value != expected {
        return Err(IdentityError::SignatureMismatch);
    }
    Ok(())
}

/// Produce a stub signature for testing.
pub fn sign_payload(identity: &AuthorIdentity, payload: &[u8]) -> Signature {
    let value = format!("{:x}-{}", payload.len(), identity.handle);
    Signature {
        signer: identity.handle.clone(),
        value,
        algorithm: "stub-ed25519".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_roundtrip() {
        let mut id = AuthorIdentity::new("alice", "Alice", "PUBKEY").unwrap();
        id.mark_verified();
        let payload = b"hello world";
        let sig = sign_payload(&id, payload);
        assert!(verify_signature(&id, payload, &sig).is_ok());
    }

    #[test]
    fn wrong_payload_fails() {
        let mut id = AuthorIdentity::new("bob", "Bob", "PUBKEY").unwrap();
        id.mark_verified();
        let sig = sign_payload(&id, b"correct");
        assert_eq!(
            verify_signature(&id, b"wrong", &sig),
            Err(IdentityError::SignatureMismatch)
        );
    }
}
