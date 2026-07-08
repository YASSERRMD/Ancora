/// An offline (air-gapped) attestation proof.
///
/// In an air-gapped environment there is no network; attestation must be done via
/// a cryptographic proof that can be verified offline. Here we simulate this with
/// a deterministic proof token computed from device state without any network calls.
#[derive(Debug, Clone)]
pub struct AirGappedProof {
    pub device_id: String,
    /// Proof token: a deterministic byte sequence derived from device state.
    pub proof_token: Vec<u8>,
    /// The tick (logical clock) at which the proof was generated.
    pub tick: u64,
    /// Nonce used to prevent replay.
    pub nonce: u64,
}

impl AirGappedProof {
    /// Generate a proof from device id, boot hash, and a nonce.
    /// All computation is purely deterministic, no network or RNG.
    pub fn generate(device_id: &str, boot_hash: &[u8], tick: u64, nonce: u64) -> Self {
        let mut proof_token = vec![0u8; 32];
        let device_bytes: Vec<u8> = device_id.bytes().collect();
        let nonce_bytes = nonce.to_le_bytes();
        let tick_bytes = tick.to_le_bytes();

        for i in 0..32 {
            let d = device_bytes
                .get(i % device_bytes.len().max(1))
                .copied()
                .unwrap_or(0);
            let b = boot_hash
                .get(i % boot_hash.len().max(1))
                .copied()
                .unwrap_or(0);
            let n = nonce_bytes[i % 8];
            let t = tick_bytes[i % 8];
            proof_token[i] = d ^ b ^ n ^ t ^ (i as u8).wrapping_mul(0x37);
        }

        Self {
            device_id: device_id.to_string(),
            proof_token,
            tick,
            nonce,
        }
    }

    /// Verify an air-gapped proof offline: re-derive and compare.
    pub fn verify(
        device_id: &str,
        boot_hash: &[u8],
        tick: u64,
        nonce: u64,
        presented_proof: &[u8],
    ) -> bool {
        let expected = Self::generate(device_id, boot_hash, tick, nonce);
        expected.proof_token == presented_proof
    }

    /// Returns the proof token as a hex string.
    pub fn token_hex(&self) -> String {
        self.proof_token
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

/// An air-gapped attestation session: bundles proof and metadata for offline transport.
#[derive(Debug, Clone)]
pub struct AirGappedAttestationBundle {
    pub proof: AirGappedProof,
    pub boot_hash: Vec<u8>,
    pub device_metadata: String,
}

impl AirGappedAttestationBundle {
    pub fn new(
        proof: AirGappedProof,
        boot_hash: Vec<u8>,
        device_metadata: impl Into<String>,
    ) -> Self {
        Self {
            proof,
            boot_hash,
            device_metadata: device_metadata.into(),
        }
    }

    /// Verify the bundle offline.
    pub fn verify_offline(&self) -> bool {
        AirGappedProof::verify(
            &self.proof.device_id,
            &self.boot_hash,
            self.proof.tick,
            self.proof.nonce,
            &self.proof.proof_token,
        )
    }

    /// Serialize to a text representation for physical transport (USB, QR code, etc.).
    pub fn to_text(&self) -> String {
        format!(
            "device_id={}\ntick={}\nnonce={}\nproof={}\nmetadata={}\n",
            self.proof.device_id,
            self.proof.tick,
            self.proof.nonce,
            self.proof.token_hex(),
            self.device_metadata,
        )
    }
}
