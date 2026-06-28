use crate::attestation::AttestationLog;
use crate::chain::BootChain;

pub struct BootStats {
    pub tenant_id: String,
    pub chain_steps: usize,
    pub attestation_count: usize,
    pub trusted_count: usize,
    pub untrusted_count: usize,
}

impl BootStats {
    pub fn from(chain: &BootChain, attestations: &AttestationLog) -> Self {
        let tenant_attestations = attestations.for_tenant(&chain.tenant_id);
        let trusted = tenant_attestations.iter().filter(|a| a.is_trusted()).count();
        let untrusted = tenant_attestations.len() - trusted;
        Self {
            tenant_id: chain.tenant_id.clone(),
            chain_steps: chain.len(),
            attestation_count: tenant_attestations.len(),
            trusted_count: trusted,
            untrusted_count: untrusted,
        }
    }

    pub fn trust_rate(&self) -> f64 {
        if self.attestation_count == 0 { return 0.0; }
        self.trusted_count as f64 / self.attestation_count as f64
    }
}
