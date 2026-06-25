use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::agent_card::AgentCard;

/// An Ed25519 signing key pair for agent identity.
pub struct AgentIdentity {
    signing_key: SigningKey,
}

impl AgentIdentity {
    /// Generate a new random identity.
    pub fn generate() -> Self {
        Self {
            signing_key: SigningKey::generate(&mut OsRng),
        }
    }

    /// Return the verifying (public) key encoded as URL-safe base64.
    pub fn public_key_b64(&self) -> String {
        URL_SAFE_NO_PAD.encode(self.signing_key.verifying_key().as_bytes())
    }

    /// Sign the canonical bytes of an agent card and return the signature as
    /// URL-safe base64.
    pub fn sign_card(&self, card: &AgentCard) -> String {
        let bytes = card.canonical_bytes();
        let sig: Signature = self.signing_key.sign(&bytes);
        URL_SAFE_NO_PAD.encode(sig.to_bytes())
    }

    /// Attach `identity_key` and `signature` to a card, returning the updated
    /// card.
    pub fn attach_to(self, mut card: AgentCard) -> AgentCard {
        card.identity_key = Some(self.public_key_b64());
        let sig = self.sign_card(&card);
        card.signature = Some(sig);
        card
    }
}

/// Verify the signature on a signed agent card.
///
/// Returns `true` if the `identity_key` and `signature` fields are present and
/// the signature is cryptographically valid over the card's canonical bytes.
pub fn verify_card(card: &AgentCard) -> bool {
    let (Some(key_b64), Some(sig_b64)) = (&card.identity_key, &card.signature) else {
        return false;
    };
    let key_bytes: [u8; 32] = match URL_SAFE_NO_PAD.decode(key_b64) {
        Ok(b) if b.len() == 32 => b.try_into().expect("len == 32"),
        _ => return false,
    };
    let sig_bytes: [u8; 64] = match URL_SAFE_NO_PAD.decode(sig_b64) {
        Ok(b) if b.len() == 64 => b.try_into().expect("len == 64"),
        _ => return false,
    };
    let Ok(vk) = VerifyingKey::from_bytes(&key_bytes) else {
        return false;
    };
    let sig = Signature::from_bytes(&sig_bytes);
    let canonical = card.canonical_bytes();
    vk.verify(&canonical, &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_card::AgentCard;

    fn sample_card() -> AgentCard {
        AgentCard::new("test-agent", "A test agent", "grpc://localhost:50051")
    }

    #[test]
    fn generated_identity_has_base64_public_key() {
        let id = AgentIdentity::generate();
        let key = id.public_key_b64();
        assert!(!key.is_empty());
        let decoded = URL_SAFE_NO_PAD.decode(&key).unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn signed_card_verifies() {
        let card = sample_card();
        let id = AgentIdentity::generate();
        let signed = id.attach_to(card);
        assert!(signed.identity_key.is_some());
        assert!(signed.signature.is_some());
        assert!(verify_card(&signed));
    }

    #[test]
    fn tampered_card_fails_verification() {
        let card = sample_card();
        let id = AgentIdentity::generate();
        let mut signed = id.attach_to(card);
        signed.description = "tampered".into();
        assert!(!verify_card(&signed));
    }

    #[test]
    fn unsigned_card_fails_verification() {
        let card = sample_card();
        assert!(!verify_card(&card));
    }

    #[test]
    fn card_with_missing_signature_fails() {
        let card = sample_card();
        let id = AgentIdentity::generate();
        let mut signed = id.attach_to(card);
        signed.signature = None;
        assert!(!verify_card(&signed));
    }

    #[test]
    fn card_with_missing_key_fails() {
        let card = sample_card();
        let id = AgentIdentity::generate();
        let mut signed = id.attach_to(card);
        signed.identity_key = None;
        assert!(!verify_card(&signed));
    }
}
