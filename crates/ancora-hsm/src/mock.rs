use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use std::collections::HashMap;

pub struct SoftHsm {
    keys: HashMap<u64, HsmKey>,
    next_handle: u64,
    operations: usize,
}

impl Default for SoftHsm {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftHsm {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            next_handle: 1,
            operations: 0,
        }
    }

    pub fn generate_key(
        &mut self,
        slot_id: u32,
        label: impl Into<String>,
        algorithm: HsmKeyAlgorithm,
        tick: u64,
    ) -> u64 {
        let handle = self.next_handle;
        self.next_handle += 1;
        let class = match algorithm {
            HsmKeyAlgorithm::Aes128 | HsmKeyAlgorithm::Aes256 => KeyClass::SecretKey,
            _ => KeyClass::PrivateKey,
        };
        self.keys.insert(
            handle,
            HsmKey::new(handle, slot_id, label, algorithm, class, tick),
        );
        self.operations += 1;
        handle
    }

    pub fn get_key(&self, handle: u64) -> Option<&HsmKey> {
        self.keys.get(&handle)
    }

    pub fn delete_key(&mut self, handle: u64) -> bool {
        if self.keys.remove(&handle).is_some() {
            self.operations += 1;
            true
        } else {
            false
        }
    }

    pub fn sign(&self, handle: u64, data: &[u8]) -> Option<Vec<u8>> {
        if self.keys.contains_key(&handle) {
            let mut sig = data.to_vec();
            sig.extend_from_slice(&handle.to_le_bytes());
            Some(sig)
        } else {
            None
        }
    }

    pub fn encrypt(&self, handle: u64, data: &[u8]) -> Option<Vec<u8>> {
        if self.keys.contains_key(&handle) {
            let mut enc = data.to_vec();
            for b in enc.iter_mut() {
                *b = b.wrapping_add(1);
            }
            Some(enc)
        } else {
            None
        }
    }

    pub fn decrypt(&self, handle: u64, data: &[u8]) -> Option<Vec<u8>> {
        if self.keys.contains_key(&handle) {
            let mut dec = data.to_vec();
            for b in dec.iter_mut() {
                *b = b.wrapping_sub(1);
            }
            Some(dec)
        } else {
            None
        }
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }
    pub fn operation_count(&self) -> usize {
        self.operations
    }
    pub fn keys_for_slot(&self, slot_id: u32) -> Vec<&HsmKey> {
        self.keys
            .values()
            .filter(|k| k.slot_id == slot_id)
            .collect()
    }
}
