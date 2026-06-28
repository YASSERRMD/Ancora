use std::collections::HashMap;

use crate::versioning::Version;

/// A trusted public key registered with the registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedKey {
    /// Identifier for this key (e.g., fingerprint or owner name).
    pub id: String,
    /// The raw key material (simplified representation; not a real crypto key).
    pub material: Vec<u8>,
}

impl TrustedKey {
    pub fn new(id: impl Into<String>, material: Vec<u8>) -> Self {
        Self {
            id: id.into(),
            material,
        }
    }
}

/// Stores trusted public keys and previously observed signatures.
///
/// In production this would use a real asymmetric scheme (ed25519, etc.).
/// Here we keep things dependency-free: a "valid" signature is the hex
/// encoding of the SHA-256-like XOR checksum of the key material XOR'd
/// with the payload name+version bytes.
#[derive(Debug, Default)]
pub struct SignatureStore {
    /// Trusted keys indexed by key id.
    keys: HashMap<String, TrustedKey>,
    /// Stored (entry name, version) -> signature pairs.
    signatures: HashMap<(String, Version), String>,
}

impl SignatureStore {
    /// Register a trusted public key.
    pub fn add_trusted_key(&mut self, key: TrustedKey) {
        self.keys.insert(key.id.clone(), key);
    }

    /// Store a signature for a given (name, version).
    pub fn store(&mut self, name: String, version: Version, sig: String) {
        self.signatures.insert((name, version), sig);
    }

    /// Retrieve the stored signature for (name, version), if any.
    pub fn get_signature(&self, name: &str, version: &Version) -> Option<&String> {
        self.signatures.get(&(name.to_string(), version.clone()))
    }

    /// Verify a signature for (name, version) against any registered trusted key.
    ///
    /// A signature is considered valid when it equals the expected token produced
    /// by `expected_sig` for at least one trusted key.
    pub fn verify(&self, name: &str, version: &Version, sig: &str) -> bool {
        if self.keys.is_empty() {
            // No keys registered: accept everything (open mode).
            return true;
        }
        self.keys.values().any(|k| {
            let expected = Self::expected_sig(k, name, version);
            sig == expected
        })
    }

    /// Produce the expected signature token for a given key + entry.
    ///
    /// This is a deterministic stand-in for a real signature scheme.
    pub fn expected_sig(key: &TrustedKey, name: &str, version: &Version) -> String {
        let mut acc: u8 = 0;
        for b in key.material.iter() {
            acc ^= b;
        }
        for b in name.as_bytes() {
            acc ^= b;
        }
        let ver_str = version.to_string();
        for b in ver_str.as_bytes() {
            acc ^= b;
        }
        format!("sig-{}-{acc:02x}", key.id)
    }

    /// List all trusted key IDs.
    pub fn trusted_key_ids(&self) -> Vec<&str> {
        self.keys.keys().map(String::as_str).collect()
    }
}
