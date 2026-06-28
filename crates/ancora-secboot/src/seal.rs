use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealResult {
    Sealed,
    AlreadySealed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnsealResult {
    Unsealed(String),
    PolicyMismatch,
    NotSealed,
}

#[derive(Debug, Clone)]
pub struct SealedBlob {
    pub id: String,
    pub tenant_id: String,
    pub data: String,
    pub required_digest: String,
    pub tick: u64,
}

pub struct SealingStore {
    blobs: HashMap<String, SealedBlob>,
}

impl SealingStore {
    pub fn new() -> Self { Self { blobs: HashMap::new() } }

    pub fn seal(
        &mut self,
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        data: impl Into<String>,
        required_digest: impl Into<String>,
        tick: u64,
    ) -> SealResult {
        let id = id.into();
        if self.blobs.contains_key(&id) {
            return SealResult::AlreadySealed;
        }
        self.blobs.insert(id.clone(), SealedBlob {
            id,
            tenant_id: tenant_id.into(),
            data: data.into(),
            required_digest: required_digest.into(),
            tick,
        });
        SealResult::Sealed
    }

    pub fn unseal(&self, id: &str, current_digest: &str) -> UnsealResult {
        match self.blobs.get(id) {
            None => UnsealResult::NotSealed,
            Some(blob) => {
                if blob.required_digest == current_digest {
                    UnsealResult::Unsealed(blob.data.clone())
                } else {
                    UnsealResult::PolicyMismatch
                }
            }
        }
    }

    pub fn count(&self) -> usize { self.blobs.len() }
    pub fn contains(&self, id: &str) -> bool { self.blobs.contains_key(id) }
}
