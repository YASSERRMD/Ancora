use crate::sbom::Sbom;
use crate::signature::SignatureStore;
use crate::provenance::ProvenanceStore;
use crate::policy::{PolicyDecision, SupplyChainPolicy};

pub struct SupplyChainReport {
    pub tenant_id: String,
    pub tick: u64,
    pub total_components: usize,
    pub signed_count: usize,
    pub unsigned_count: usize,
    pub provenance_count: usize,
    pub denied_count: usize,
    pub proprietary_count: usize,
}

impl SupplyChainReport {
    pub fn generate(
        sbom: &Sbom,
        sigs: &SignatureStore,
        provenance: &ProvenanceStore,
        policy: &SupplyChainPolicy,
        tick: u64,
    ) -> Self {
        let mut signed = 0;
        let mut with_prov = 0;
        let mut denied = 0;
        for c in &sbom.components {
            let has_sig = sigs.has_signature(&c.id);
            let has_prov = provenance.has_provenance(&c.id);
            if has_sig { signed += 1; }
            if has_prov { with_prov += 1; }
            if let PolicyDecision::Deny(_) = policy.check_component(c, has_sig, has_prov) {
                denied += 1;
            }
        }
        Self {
            tenant_id: sbom.tenant_id.clone(),
            tick,
            total_components: sbom.component_count(),
            signed_count: signed,
            unsigned_count: sbom.component_count().saturating_sub(signed),
            provenance_count: with_prov,
            denied_count: denied,
            proprietary_count: sbom.proprietary_count(),
        }
    }

    pub fn is_compliant(&self) -> bool {
        self.denied_count == 0
    }

    pub fn sign_rate(&self) -> f64 {
        if self.total_components == 0 { return 0.0; }
        self.signed_count as f64 / self.total_components as f64
    }
}
