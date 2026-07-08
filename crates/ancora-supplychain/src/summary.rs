use crate::policy::SupplyChainPolicy;
use crate::provenance::ProvenanceStore;
use crate::report::SupplyChainReport;
use crate::sbom::Sbom;
use crate::signature::SignatureStore;
use crate::stats::SbomStats;
use crate::validator::{SbomIssue, SbomValidator};

pub struct SupplyChainSummary {
    pub tenant_id: String,
    pub tick: u64,
    pub total_components: usize,
    pub signed_count: usize,
    pub oss_count: usize,
    pub is_compliant: bool,
    pub validation_issue_count: usize,
    pub sign_rate: f64,
    pub oss_rate: f64,
}

impl SupplyChainSummary {
    pub fn generate(
        sbom: &Sbom,
        sigs: &SignatureStore,
        prov: &ProvenanceStore,
        policy: &SupplyChainPolicy,
        tick: u64,
    ) -> Self {
        let report = SupplyChainReport::generate(sbom, sigs, prov, policy, tick);
        let stats = SbomStats::from(sbom);
        let issues: Vec<SbomIssue> = SbomValidator::validate(sbom, sigs, prov, false, false);
        Self {
            tenant_id: sbom.tenant_id.clone(),
            tick,
            total_components: report.total_components,
            signed_count: report.signed_count,
            oss_count: stats.open_source_count,
            is_compliant: report.is_compliant(),
            validation_issue_count: issues.len(),
            sign_rate: report.sign_rate(),
            oss_rate: stats.oss_rate(),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.is_compliant && self.validation_issue_count == 0
    }
}
