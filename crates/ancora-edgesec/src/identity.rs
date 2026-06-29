use std::collections::HashMap;
use std::fmt;

/// Unique identifier for an edge device.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A key pair for an edge device: a simulated public and private key (byte blobs in pure std).
#[derive(Debug, Clone)]
pub struct DeviceKeyPair {
    pub device_id: DeviceId,
    /// Simulated public key (32 bytes, deterministic from device_id for tests).
    pub public_key: Vec<u8>,
    /// Simulated private key (32 bytes, deterministic from device_id for tests).
    pub private_key: Vec<u8>,
    /// Indicates whether this key pair has been revoked.
    pub revoked: bool,
}

impl DeviceKeyPair {
    /// Generate a deterministic key pair from a device id (pure std, no crypto deps).
    pub fn generate(device_id: DeviceId) -> Self {
        let seed: Vec<u8> = device_id
            .0
            .bytes()
            .enumerate()
            .map(|(i, b)| b.wrapping_add(i as u8))
            .collect();
        let mut public_key = vec![0u8; 32];
        let mut private_key = vec![0u8; 32];
        for i in 0..32 {
            public_key[i] = seed.get(i % seed.len()).copied().unwrap_or(0xAA) ^ 0x5A;
            private_key[i] = seed.get(i % seed.len()).copied().unwrap_or(0x55) ^ 0xA5;
        }
        Self {
            device_id,
            public_key,
            private_key,
            revoked: false,
        }
    }

    /// Return a hex-encoded representation of the public key.
    pub fn public_key_hex(&self) -> String {
        self.public_key
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

/// Registry of all device identity key pairs.
pub struct DeviceIdentityRegistry {
    keys: HashMap<DeviceId, DeviceKeyPair>,
}

impl DeviceIdentityRegistry {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    /// Register a new device and generate its key pair.
    pub fn register(&mut self, device_id: DeviceId) -> &DeviceKeyPair {
        let kp = DeviceKeyPair::generate(device_id.clone());
        self.keys.insert(device_id.clone(), kp);
        self.keys.get(&device_id).unwrap()
    }

    /// Look up a device key pair by id.
    pub fn get(&self, device_id: &DeviceId) -> Option<&DeviceKeyPair> {
        self.keys.get(device_id)
    }

    /// Revoke a device key pair.
    pub fn revoke(&mut self, device_id: &DeviceId) -> bool {
        if let Some(kp) = self.keys.get_mut(device_id) {
            kp.revoked = true;
            true
        } else {
            false
        }
    }

    /// Returns true if the device's key pair is revoked.
    pub fn is_revoked(&self, device_id: &DeviceId) -> bool {
        self.keys.get(device_id).map(|kp| kp.revoked).unwrap_or(false)
    }

    /// Number of registered devices.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Verify ownership: returns true if the presented public key matches the device's registered key and is not revoked.
    pub fn verify_identity(&self, device_id: &DeviceId, presented_public_key: &[u8]) -> bool {
        if let Some(kp) = self.keys.get(device_id) {
            !kp.revoked && kp.public_key == presented_public_key
        } else {
            false
        }
    }
}
