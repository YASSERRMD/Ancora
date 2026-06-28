use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    Ed25519,
    EcdsaP256,
    Rsa2048,
    Hmac256,
}

impl fmt::Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SignatureAlgorithm::Ed25519 => "ED25519",
            SignatureAlgorithm::EcdsaP256 => "ECDSA-P256",
            SignatureAlgorithm::Rsa2048 => "RSA-2048",
            SignatureAlgorithm::Hmac256 => "HMAC-256",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    Valid,
    Invalid(String),
    Missing,
}

#[derive(Debug, Clone)]
pub struct ComponentSignature {
    pub component_id: String,
    pub algorithm: SignatureAlgorithm,
    pub signer: String,
    pub signature: String,
    pub tick: u64,
}

impl ComponentSignature {
    pub fn new(
        component_id: impl Into<String>,
        algorithm: SignatureAlgorithm,
        signer: impl Into<String>,
        signature: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            component_id: component_id.into(),
            algorithm,
            signer: signer.into(),
            signature: signature.into(),
            tick,
        }
    }
}

pub struct SignatureStore {
    sigs: HashMap<String, ComponentSignature>,
}

impl SignatureStore {
    pub fn new() -> Self { Self { sigs: HashMap::new() } }

    pub fn register(&mut self, sig: ComponentSignature) {
        self.sigs.insert(sig.component_id.clone(), sig);
    }

    pub fn verify(&self, component_id: &str, expected_signature: &str) -> VerificationResult {
        match self.sigs.get(component_id) {
            None => VerificationResult::Missing,
            Some(sig) => {
                if sig.signature == expected_signature {
                    VerificationResult::Valid
                } else {
                    VerificationResult::Invalid(format!("signature mismatch for {}", component_id))
                }
            }
        }
    }

    pub fn has_signature(&self, component_id: &str) -> bool {
        self.sigs.contains_key(component_id)
    }

    pub fn count(&self) -> usize { self.sigs.len() }

    pub fn by_signer(&self, signer: &str) -> Vec<&ComponentSignature> {
        self.sigs.values().filter(|s| s.signer == signer).collect()
    }
}
