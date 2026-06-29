//! Encrypted sync transport layer.
//!
//! Wraps serialised [`SyncRequest`] / [`SyncResponse`] payloads in an
//! authenticated-encryption envelope.  The implementation uses a simple
//! XOR-based stream cipher seeded from a pre-shared key for illustration
//! (a real deployment would use AES-GCM or ChaCha20-Poly1305).

use crate::protocol::{SyncRequest, SyncResponse};
use serde::{Deserialize, Serialize};

/// An encrypted, integrity-protected transport envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    /// Nonce used during encryption (8 bytes).
    pub nonce: [u8; 8],
    /// MAC / tag computed over the ciphertext.
    pub tag: u64,
    /// Encrypted ciphertext.
    pub ciphertext: Vec<u8>,
}

impl EncryptedEnvelope {
    /// Verify the MAC tag before decrypting.
    pub fn verify_tag(&self) -> bool {
        compute_tag(&self.ciphertext, &self.nonce) == self.tag
    }
}

/// Compute a simple tag over ciphertext + nonce.
fn compute_tag(data: &[u8], nonce: &[u8; 8]) -> u64 {
    const FNV_PRIME: u64 = 1_099_511_628_211;
    const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
    let mut hash = FNV_OFFSET;
    for byte in data.iter().chain(nonce.iter()) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// XOR keystream generator seeded from a pre-shared key.
fn keystream(key: &[u8], nonce: &[u8; 8], len: usize) -> Vec<u8> {
    let seed: u64 = {
        const FNV_PRIME: u64 = 1_099_511_628_211;
        const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;
        let mut h = FNV_OFFSET;
        for b in key.iter().chain(nonce.iter()) {
            h ^= u64::from(*b);
            h = h.wrapping_mul(FNV_PRIME);
        }
        h
    };
    // Simple LCG to expand seed into a keystream.
    let mut state = seed;
    let mut stream = Vec::with_capacity(len);
    while stream.len() < len {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        stream.extend_from_slice(&state.to_le_bytes());
    }
    stream.truncate(len);
    stream
}

/// Encrypt serialised bytes with the given pre-shared key and nonce.
pub fn encrypt(plaintext: &[u8], key: &[u8], nonce: [u8; 8]) -> EncryptedEnvelope {
    let ks = keystream(key, &nonce, plaintext.len());
    let ciphertext: Vec<u8> = plaintext.iter().zip(ks.iter()).map(|(p, k)| p ^ k).collect();
    let tag = compute_tag(&ciphertext, &nonce);
    EncryptedEnvelope { nonce, ciphertext, tag }
}

/// Decrypt an envelope; returns `None` if the MAC check fails.
pub fn decrypt(envelope: &EncryptedEnvelope, key: &[u8]) -> Option<Vec<u8>> {
    if !envelope.verify_tag() {
        return None;
    }
    let ks = keystream(key, &envelope.nonce, envelope.ciphertext.len());
    Some(envelope.ciphertext.iter().zip(ks.iter()).map(|(c, k)| c ^ k).collect())
}

/// Wrap a [`SyncRequest`] in an encrypted envelope.
pub fn seal_request(request: &SyncRequest, key: &[u8], nonce: [u8; 8]) -> Result<EncryptedEnvelope, serde_json::Error> {
    let plaintext = serde_json::to_vec(request)?;
    Ok(encrypt(&plaintext, key, nonce))
}

/// Unwrap and parse an encrypted [`SyncRequest`] from an envelope.
pub fn open_request(envelope: &EncryptedEnvelope, key: &[u8]) -> Option<SyncRequest> {
    let plaintext = decrypt(envelope, key)?;
    serde_json::from_slice(&plaintext).ok()
}

/// Wrap a [`SyncResponse`] in an encrypted envelope.
pub fn seal_response(response: &SyncResponse, key: &[u8], nonce: [u8; 8]) -> Result<EncryptedEnvelope, serde_json::Error> {
    let plaintext = serde_json::to_vec(response)?;
    Ok(encrypt(&plaintext, key, nonce))
}

/// Unwrap and parse an encrypted [`SyncResponse`] from an envelope.
pub fn open_response(envelope: &EncryptedEnvelope, key: &[u8]) -> Option<SyncResponse> {
    let plaintext = decrypt(envelope, key)?;
    serde_json::from_slice(&plaintext).ok()
}
