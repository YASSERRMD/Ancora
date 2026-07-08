use crate::key::{CryptoKey, KeyAlgorithm, KeyPurpose};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmBackend {
    Software,
    CloudKms,
    Pkcs11,
    Tpm,
}

impl std::fmt::Display for HsmBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HsmBackend::Software => write!(f, "SOFTWARE"),
            HsmBackend::CloudKms => write!(f, "CLOUD_KMS"),
            HsmBackend::Pkcs11 => write!(f, "PKCS11"),
            HsmBackend::Tpm => write!(f, "TPM"),
        }
    }
}

pub struct HsmConfig {
    pub backend: HsmBackend,
    pub slot_id: u32,
}

impl HsmConfig {
    pub fn software() -> Self {
        Self {
            backend: HsmBackend::Software,
            slot_id: 0,
        }
    }
    pub fn cloud_kms(slot_id: u32) -> Self {
        Self {
            backend: HsmBackend::CloudKms,
            slot_id,
        }
    }

    pub fn is_hardware_backed(&self) -> bool {
        self.backend != HsmBackend::Software
    }
}

pub struct HsmStub {
    pub config: HsmConfig,
    monotonic: u64,
}

impl HsmStub {
    pub fn new(config: HsmConfig) -> Self {
        Self {
            config,
            monotonic: 0,
        }
    }

    pub fn generate_key(
        &mut self,
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        algorithm: KeyAlgorithm,
        purpose: KeyPurpose,
        tick: u64,
    ) -> CryptoKey {
        self.monotonic += 1;
        let material = format!("hsm-{}-{}", self.config.backend, self.monotonic);
        CryptoKey::new(id, tenant_id, algorithm, purpose, tick, material)
    }

    pub fn backend(&self) -> &HsmBackend {
        &self.config.backend
    }
}
