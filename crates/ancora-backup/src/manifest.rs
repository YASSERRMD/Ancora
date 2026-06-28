use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// Backup manifest records what's in the archive and a checksum for integrity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub version: u32,
    pub created_at_secs: u64,
    pub entry_count: usize,
    pub max_seq: u64,
    pub checksum_sha256: String,
    pub encrypted: bool,
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

impl BackupManifest {
    pub fn new(data: &[u8], entry_count: usize, max_seq: u64, encrypted: bool, now: u64) -> Self {
        Self {
            version: 1,
            created_at_secs: now,
            entry_count,
            max_seq,
            checksum_sha256: sha256_hex(data),
            encrypted,
        }
    }

    pub fn verify(&self, data: &[u8]) -> bool {
        sha256_hex(data) == self.checksum_sha256
    }
}
