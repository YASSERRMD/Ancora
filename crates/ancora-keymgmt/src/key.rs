use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAlgorithm {
    Aes256,
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    Ed25519,
    Hmac256,
}

impl fmt::Display for KeyAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyAlgorithm::Aes256 => write!(f, "AES-256"),
            KeyAlgorithm::Rsa2048 => write!(f, "RSA-2048"),
            KeyAlgorithm::Rsa4096 => write!(f, "RSA-4096"),
            KeyAlgorithm::EcdsaP256 => write!(f, "ECDSA-P256"),
            KeyAlgorithm::EcdsaP384 => write!(f, "ECDSA-P384"),
            KeyAlgorithm::Ed25519 => write!(f, "ED25519"),
            KeyAlgorithm::Hmac256 => write!(f, "HMAC-256"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyPurpose {
    Encryption,
    Signing,
    Authentication,
    KeyWrapping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyStatus {
    Active,
    Inactive,
    Compromised,
    Destroyed,
    PendingDeletion,
}

impl fmt::Display for KeyStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyStatus::Active => write!(f, "ACTIVE"),
            KeyStatus::Inactive => write!(f, "INACTIVE"),
            KeyStatus::Compromised => write!(f, "COMPROMISED"),
            KeyStatus::Destroyed => write!(f, "DESTROYED"),
            KeyStatus::PendingDeletion => write!(f, "PENDING_DELETION"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CryptoKey {
    pub id: String,
    pub tenant_id: String,
    pub algorithm: KeyAlgorithm,
    pub purpose: KeyPurpose,
    pub status: KeyStatus,
    pub version: u32,
    pub created_tick: u64,
    pub expires_tick: Option<u64>,
    pub metadata: HashMap<String, String>,
    pub key_material: String,
}

impl CryptoKey {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        algorithm: KeyAlgorithm,
        purpose: KeyPurpose,
        created_tick: u64,
        key_material: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            algorithm,
            purpose,
            status: KeyStatus::Active,
            version: 1,
            created_tick,
            expires_tick: None,
            metadata: HashMap::new(),
            key_material: key_material.into(),
        }
    }

    pub fn with_expiry(mut self, expires_tick: u64) -> Self {
        self.expires_tick = Some(expires_tick);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn is_active(&self) -> bool {
        self.status == KeyStatus::Active
    }
    pub fn is_expired(&self, current_tick: u64) -> bool {
        self.expires_tick.is_some_and(|e| current_tick >= e)
    }

    pub fn deactivate(&mut self) {
        self.status = KeyStatus::Inactive;
    }
    pub fn mark_compromised(&mut self) {
        self.status = KeyStatus::Compromised;
    }
    pub fn schedule_deletion(&mut self) {
        self.status = KeyStatus::PendingDeletion;
    }
    pub fn destroy(&mut self) {
        self.status = KeyStatus::Destroyed;
        self.key_material = String::new();
    }
}
