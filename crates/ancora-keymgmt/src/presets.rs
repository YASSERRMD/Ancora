use crate::builder::KeyBuilder;
use crate::key::{CryptoKey, KeyAlgorithm, KeyPurpose};

pub fn aes256_encryption_key(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> CryptoKey {
    KeyBuilder::new(id, tenant_id)
        .algorithm(KeyAlgorithm::Aes256)
        .purpose(KeyPurpose::Encryption)
        .tick(tick)
        .material("preset-aes256")
        .build()
}

pub fn ed25519_signing_key(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> CryptoKey {
    KeyBuilder::new(id, tenant_id)
        .algorithm(KeyAlgorithm::Ed25519)
        .purpose(KeyPurpose::Signing)
        .tick(tick)
        .material("preset-ed25519")
        .build()
}

pub fn rsa2048_auth_key(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> CryptoKey {
    KeyBuilder::new(id, tenant_id)
        .algorithm(KeyAlgorithm::Rsa2048)
        .purpose(KeyPurpose::Authentication)
        .tick(tick)
        .material("preset-rsa2048")
        .build()
}

pub fn hmac256_signing_key(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> CryptoKey {
    KeyBuilder::new(id, tenant_id)
        .algorithm(KeyAlgorithm::Hmac256)
        .purpose(KeyPurpose::Signing)
        .tick(tick)
        .material("preset-hmac256")
        .build()
}

pub fn ephemeral_key(id: impl Into<String>, tenant_id: impl Into<String>, tick: u64, ttl: u64) -> CryptoKey {
    KeyBuilder::new(id, tenant_id)
        .algorithm(KeyAlgorithm::Aes256)
        .purpose(KeyPurpose::Encryption)
        .tick(tick)
        .expires_at(tick + ttl)
        .material("preset-ephemeral")
        .build()
}
