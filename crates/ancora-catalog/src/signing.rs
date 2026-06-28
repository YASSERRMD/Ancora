/// A catalog signature produced by a signing key.
/// In production this would use ed25519 or similar; here we use a simple
/// HMAC-SHA256-like stub that operates entirely in std.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSignature {
    /// Hex-encoded signature bytes.
    pub value: String,
    /// Identifier of the key that produced the signature.
    pub key_id: String,
}

/// A signing key backed by a raw secret (bytes).
#[derive(Debug, Clone)]
pub struct SigningKey {
    pub key_id: String,
    secret: Vec<u8>,
}

impl SigningKey {
    pub fn new(key_id: impl Into<String>, secret: Vec<u8>) -> Self {
        Self {
            key_id: key_id.into(),
            secret,
        }
    }

    /// Sign `payload` and return a [`CatalogSignature`].
    /// The algorithm is a simple XOR-based accumulator so we have no external deps.
    pub fn sign(&self, payload: &[u8]) -> CatalogSignature {
        let hash = simple_hash(&self.secret, payload);
        CatalogSignature {
            value: hex_encode(&hash),
            key_id: self.key_id.clone(),
        }
    }

    /// Verify that `sig` was produced by this key over `payload`.
    pub fn verify(&self, payload: &[u8], sig: &CatalogSignature) -> bool {
        if sig.key_id != self.key_id {
            return false;
        }
        let expected = simple_hash(&self.secret, payload);
        hex_encode(&expected) == sig.value
    }
}

/// A catalog payload that can be signed or verified.
#[derive(Debug, Clone)]
pub struct SignedCatalog {
    /// Canonical bytes of the catalog (e.g. serialised JSON/TOML).
    pub payload: Vec<u8>,
    pub signature: Option<CatalogSignature>,
}

impl SignedCatalog {
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            payload,
            signature: None,
        }
    }

    pub fn sign(&mut self, key: &SigningKey) {
        self.signature = Some(key.sign(&self.payload));
    }

    pub fn verify(&self, key: &SigningKey) -> bool {
        match &self.signature {
            None => false,
            Some(sig) => key.verify(&self.payload, sig),
        }
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}

// ---------------------------------------------------------------------------
// Internal helpers (no external deps)
// ---------------------------------------------------------------------------

/// A 32-byte keyed hash using XOR folding - deterministic and collision-resistant
/// enough for testing purposes.
fn simple_hash(key: &[u8], payload: &[u8]) -> [u8; 32] {
    let mut state = [0u8; 32];
    // Mix in the key.
    for (i, &b) in key.iter().enumerate() {
        state[i % 32] = state[i % 32].wrapping_add(b).wrapping_add(i as u8);
    }
    // Mix in the payload with a different stride.
    for (i, &b) in payload.iter().enumerate() {
        let idx = (i * 7 + 3) % 32;
        state[idx] = state[idx]
            .wrapping_add(b)
            .wrapping_add((i % 251) as u8)
            .rotate_left(1);
    }
    // Final diffusion pass.
    for i in 1..32 {
        state[i] = state[i].wrapping_add(state[i - 1]);
    }
    state
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
