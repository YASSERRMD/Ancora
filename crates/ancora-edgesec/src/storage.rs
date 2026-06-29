use std::collections::HashMap;

/// A simple XOR cipher key (16 bytes) for simulated encrypted local storage.
#[derive(Debug, Clone)]
pub struct StorageKey {
    pub bytes: [u8; 16],
}

impl StorageKey {
    /// Create a storage key from a device id (deterministic, pure std).
    pub fn from_device_id(device_id: &str) -> Self {
        let mut bytes = [0u8; 16];
        for (i, b) in device_id.bytes().enumerate() {
            bytes[i % 16] ^= b.wrapping_add(i as u8);
        }
        // Ensure no zero bytes (make key non-trivial).
        for b in bytes.iter_mut() {
            if *b == 0 {
                *b = 0xDE;
            }
        }
        Self { bytes }
    }

    /// XOR-encrypt/decrypt data (symmetric).
    pub fn apply(&self, data: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, b)| b ^ self.bytes[i % 16])
            .collect()
    }
}

/// A record in the encrypted local storage.
#[derive(Debug, Clone)]
pub struct StorageEntry {
    /// Encrypted ciphertext.
    pub ciphertext: Vec<u8>,
}

/// Encrypted local storage for an edge device.
pub struct EncryptedLocalStorage {
    key: StorageKey,
    entries: HashMap<String, StorageEntry>,
}

impl EncryptedLocalStorage {
    pub fn new(key: StorageKey) -> Self {
        Self {
            key,
            entries: HashMap::new(),
        }
    }

    /// Store a plaintext value under a key (encrypts before storing).
    pub fn put(&mut self, k: impl Into<String>, plaintext: &[u8]) {
        let ciphertext = self.key.apply(plaintext);
        self.entries.insert(k.into(), StorageEntry { ciphertext });
    }

    /// Retrieve and decrypt a value by key. Returns None if not found.
    pub fn get(&self, k: &str) -> Option<Vec<u8>> {
        self.entries.get(k).map(|e| self.key.apply(&e.ciphertext))
    }

    /// Returns the raw (encrypted) ciphertext for a key. Useful for verifying encryption at rest.
    pub fn get_ciphertext(&self, k: &str) -> Option<&[u8]> {
        self.entries.get(k).map(|e| e.ciphertext.as_slice())
    }

    /// Remove a key from storage.
    pub fn remove(&mut self, k: &str) -> bool {
        self.entries.remove(k).is_some()
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if a key exists.
    pub fn contains_key(&self, k: &str) -> bool {
        self.entries.contains_key(k)
    }
}
