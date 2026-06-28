use crate::sbom::Sbom;
use crate::signature::SignatureStore;
use crate::provenance::ProvenanceStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SbomIssue {
    EmptySbom,
    UnsignedComponent(String),
    MissingProvenance(String),
    EmptyDigest(String),
}

impl std::fmt::Display for SbomIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomIssue::EmptySbom => write!(f, "SBOM has no components"),
            SbomIssue::UnsignedComponent(id) => write!(f, "component {} has no signature", id),
            SbomIssue::MissingProvenance(id) => write!(f, "component {} has no provenance record", id),
            SbomIssue::EmptyDigest(id) => write!(f, "component {} has empty digest", id),
        }
    }
}

pub struct SbomValidator;

impl SbomValidator {
    pub fn validate(
        sbom: &Sbom,
        sigs: &SignatureStore,
        prov: &ProvenanceStore,
        require_signatures: bool,
        require_provenance: bool,
    ) -> Vec<SbomIssue> {
        let mut issues = Vec::new();
        if sbom.component_count() == 0 {
            issues.push(SbomIssue::EmptySbom);
            return issues;
        }
        for c in &sbom.components {
            if c.digest.is_empty() {
                issues.push(SbomIssue::EmptyDigest(c.id.clone()));
            }
            if require_signatures && !sigs.has_signature(&c.id) {
                issues.push(SbomIssue::UnsignedComponent(c.id.clone()));
            }
            if require_provenance && !prov.has_provenance(&c.id) {
                issues.push(SbomIssue::MissingProvenance(c.id.clone()));
            }
        }
        issues
    }

    pub fn is_valid(
        sbom: &Sbom,
        sigs: &SignatureStore,
        prov: &ProvenanceStore,
        require_signatures: bool,
        require_provenance: bool,
    ) -> bool {
        Self::validate(sbom, sigs, prov, require_signatures, require_provenance).is_empty()
    }
}
