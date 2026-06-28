use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HsmKeyAlgorithm {
    Aes128,
    Aes256,
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
}

impl fmt::Display for HsmKeyAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HsmKeyAlgorithm::Aes128 => "AES-128",
            HsmKeyAlgorithm::Aes256 => "AES-256",
            HsmKeyAlgorithm::Rsa2048 => "RSA-2048",
            HsmKeyAlgorithm::Rsa4096 => "RSA-4096",
            HsmKeyAlgorithm::EcdsaP256 => "ECDSA-P256",
            HsmKeyAlgorithm::EcdsaP384 => "ECDSA-P384",
            HsmKeyAlgorithm::Ed25519 => "ED25519",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyClass {
    SecretKey,
    PublicKey,
    PrivateKey,
}

impl fmt::Display for KeyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyClass::SecretKey => "SECRET",
            KeyClass::PublicKey => "PUBLIC",
            KeyClass::PrivateKey => "PRIVATE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct HsmKey {
    pub handle: u64,
    pub slot_id: u32,
    pub label: String,
    pub algorithm: HsmKeyAlgorithm,
    pub class: KeyClass,
    pub extractable: bool,
    pub sensitive: bool,
    pub created_tick: u64,
    pub attributes: HashMap<String, String>,
}

impl HsmKey {
    pub fn new(handle: u64, slot_id: u32, label: impl Into<String>, algorithm: HsmKeyAlgorithm, class: KeyClass, tick: u64) -> Self {
        Self {
            handle,
            slot_id,
            label: label.into(),
            algorithm,
            class,
            extractable: false,
            sensitive: true,
            created_tick: tick,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attribute(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.attributes.insert(k.into(), v.into()); self
    }
}
