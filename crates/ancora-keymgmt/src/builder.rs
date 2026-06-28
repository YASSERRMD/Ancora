use crate::key::{CryptoKey, KeyAlgorithm, KeyPurpose};

pub struct KeyBuilder {
    id: String,
    tenant_id: String,
    algorithm: KeyAlgorithm,
    purpose: KeyPurpose,
    created_tick: u64,
    expires_tick: Option<u64>,
    key_material: String,
}

impl KeyBuilder {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            algorithm: KeyAlgorithm::Aes256,
            purpose: KeyPurpose::Encryption,
            created_tick: 0,
            expires_tick: None,
            key_material: "placeholder-material".to_string(),
        }
    }

    pub fn algorithm(mut self, alg: KeyAlgorithm) -> Self { self.algorithm = alg; self }
    pub fn purpose(mut self, p: KeyPurpose) -> Self { self.purpose = p; self }
    pub fn tick(mut self, t: u64) -> Self { self.created_tick = t; self }
    pub fn expires_at(mut self, t: u64) -> Self { self.expires_tick = Some(t); self }
    pub fn material(mut self, m: impl Into<String>) -> Self { self.key_material = m.into(); self }

    pub fn build(self) -> CryptoKey {
        let mut k = CryptoKey::new(self.id, self.tenant_id, self.algorithm, self.purpose, self.created_tick, self.key_material);
        if let Some(e) = self.expires_tick { k = k.with_expiry(e); }
        k
    }
}
