//! Plugin signature verification.
//!
//! Plugins must carry a cryptographic signature over their content.  The host
//! verifies the signature against a trusted set of public keys before loading
//! the plugin.  In strict mode unsigned plugins are rejected outright.

/// Controls whether the host requires plugin signatures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignaturePolicy {
    /// Plugin signatures are mandatory; unsigned plugins are rejected.
    Required,
    /// Signatures are verified when present but unsigned plugins are allowed
    /// (useful during development).
    Optional,
    /// Signature checking is completely disabled (not recommended for production).
    Disabled,
}

/// A detached signature over plugin content.
#[derive(Debug, Clone)]
pub struct PluginSignature {
    /// Identifier of the key used to produce this signature.
    pub key_id: String,
    /// The raw signature bytes (e.g., an Ed25519 or ECDSA signature).
    pub signature_bytes: Vec<u8>,
}

/// A trusted public key used to verify plugin signatures.
#[derive(Debug, Clone)]
pub struct TrustedKey {
    /// Unique identifier for this key.
    pub key_id: String,
    /// The public key material.  In a real implementation this would be a
    /// `VerifyingKey` from a crypto library.
    pub public_key_bytes: Vec<u8>,
}

/// Error returned when signature verification fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureError {
    /// The plugin has no signature but the policy requires one.
    MissingSignature,
    /// The key referenced by the signature is not in the trust store.
    UnknownKey(String),
    /// The signature bytes do not validate against the plugin content and key.
    InvalidSignature,
}

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSignature => {
                write!(f, "plugin is unsigned and policy requires a signature")
            }
            Self::UnknownKey(kid) => write!(f, "unknown signing key: {}", kid),
            Self::InvalidSignature => write!(f, "signature is invalid"),
        }
    }
}

/// The signature verifier holds the trust store and applies policy.
#[derive(Debug, Default)]
pub struct SignatureVerifier {
    trusted_keys: Vec<TrustedKey>,
}

impl SignatureVerifier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a public key as trusted.
    pub fn add_trusted_key(&mut self, key: TrustedKey) {
        self.trusted_keys.push(key);
    }

    /// Verify `sig` over `content` according to `policy`.
    ///
    /// In a real implementation this would invoke a constant-time signature
    /// verification primitive.  Here we use a simple byte-equality check so
    /// the logic is correct-by-construction and does not require external crates.
    pub fn verify(
        &self,
        content: &[u8],
        sig: Option<&PluginSignature>,
        policy: &SignaturePolicy,
    ) -> Result<(), SignatureError> {
        match policy {
            SignaturePolicy::Disabled => return Ok(()),
            SignaturePolicy::Optional if sig.is_none() => return Ok(()),
            SignaturePolicy::Required if sig.is_none() => {
                return Err(SignatureError::MissingSignature)
            }
            _ => {}
        }

        let sig = sig.expect("sig is Some at this point");

        // Look up the key.
        let key = self
            .trusted_keys
            .iter()
            .find(|k| k.key_id == sig.key_id)
            .ok_or_else(|| SignatureError::UnknownKey(sig.key_id.clone()))?;

        // Stub verification: signature must equal SHA-256(key || content).
        // In production replace with a real crypto primitive.
        let expected = Self::stub_sign(&key.public_key_bytes, content);
        if sig.signature_bytes != expected {
            return Err(SignatureError::InvalidSignature);
        }

        Ok(())
    }

    /// Produce a stub "signature" for testing purposes only.
    ///
    /// This is intentionally trivial and MUST NOT be used in production.
    pub fn stub_sign(key_bytes: &[u8], content: &[u8]) -> Vec<u8> {
        // XOR-fold key into content to produce a deterministic test vector.
        let mut out = content.to_vec();
        for (i, b) in key_bytes.iter().cycle().zip(out.iter_mut()) {
            *b ^= i;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_verifier() -> (SignatureVerifier, TrustedKey) {
        let key = TrustedKey {
            key_id: "key-1".into(),
            public_key_bytes: vec![0xDE, 0xAD, 0xBE, 0xEF],
        };
        let mut v = SignatureVerifier::new();
        v.add_trusted_key(key.clone());
        (v, key)
    }

    #[test]
    fn valid_signature_accepted() {
        let (v, key) = make_verifier();
        let content = b"plugin bytes";
        let sig_bytes = SignatureVerifier::stub_sign(&key.public_key_bytes, content);
        let sig = PluginSignature {
            key_id: "key-1".into(),
            signature_bytes: sig_bytes,
        };
        assert!(v
            .verify(content, Some(&sig), &SignaturePolicy::Required)
            .is_ok());
    }

    #[test]
    fn missing_signature_rejected_in_strict_mode() {
        let (v, _) = make_verifier();
        let err = v
            .verify(b"plugin", None, &SignaturePolicy::Required)
            .unwrap_err();
        assert_eq!(err, SignatureError::MissingSignature);
    }

    #[test]
    fn unknown_key_rejected() {
        let (v, _) = make_verifier();
        let sig = PluginSignature {
            key_id: "unknown-key".into(),
            signature_bytes: vec![1, 2, 3],
        };
        let err = v
            .verify(b"plugin", Some(&sig), &SignaturePolicy::Required)
            .unwrap_err();
        assert!(matches!(err, SignatureError::UnknownKey(_)));
    }

    #[test]
    fn disabled_policy_skips_verification() {
        let v = SignatureVerifier::new(); // no trusted keys
        assert!(v
            .verify(b"anything", None, &SignaturePolicy::Disabled)
            .is_ok());
    }
}
