use crate::attestation::AttestationLog;
use crate::chain::{BootChain, ChainStatus};
use crate::evaluator::{IntegrityDecision, IntegrityEvaluator};
use crate::policy::BootPolicy;

pub struct IntegrityReport {
    pub tenant_id: String,
    pub node_id: String,
    pub tick: u64,
    pub chain_length: usize,
    pub chain_status: ChainStatus,
    pub decision: IntegrityDecision,
    pub attestation_count: usize,
    pub trusted_attestations: usize,
}

impl IntegrityReport {
    pub fn generate(
        policy: &BootPolicy,
        chain: &BootChain,
        attestations: &AttestationLog,
        tick: u64,
    ) -> Self {
        let decision = IntegrityEvaluator::evaluate(policy, chain);
        let node_attestations: Vec<_> = attestations.for_node(&chain.node_id);
        let trusted = node_attestations.iter().filter(|a| a.is_trusted()).count();
        Self {
            tenant_id: chain.tenant_id.clone(),
            node_id: chain.node_id.clone(),
            tick,
            chain_length: chain.len(),
            chain_status: chain.status(),
            decision,
            attestation_count: node_attestations.len(),
            trusted_attestations: trusted,
        }
    }

    pub fn is_fully_trusted(&self) -> bool {
        self.decision.is_pass() && self.chain_status == ChainStatus::Valid
    }
}
