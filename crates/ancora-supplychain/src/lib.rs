//! Supply chain security with dependency signing, SBOM generation, and provenance tracking for Ancora.
//!
//! Core types: [`Component`], [`Sbom`], [`ComponentSignature`], [`ProvenanceRecord`].
//! Policy: [`SupplyChainPolicy`] with license deny-lists, signature, provenance requirements.
//! Audit: [`SupplyChainAuditLog`] tracking all supply chain events.
//! Report: [`SupplyChainReport`] aggregating compliance across SBOM components.
//! Stats: [`SbomStats`] with OSS rate and license distribution.
pub mod audit;
pub mod builder;
pub mod component;
pub mod policy;
pub mod provenance;
pub mod query;
pub mod report;
pub mod sbom;
pub mod signature;
pub mod stats;

pub use audit::{SupplyChainAuditEntry, SupplyChainAuditLog, SupplyChainEvent};
pub use builder::ComponentBuilder;
pub use component::{Component, ComponentKind, License};
pub use policy::{PolicyDecision, SupplyChainPolicy};
pub use provenance::{ProvenanceKind, ProvenanceRecord, ProvenanceStore};
pub use query::ComponentQuery;
pub use report::SupplyChainReport;
pub use sbom::{Sbom, SbomFormat, SbomStore};
pub use signature::{ComponentSignature, SignatureAlgorithm, SignatureStore, VerificationResult};
pub use stats::SbomStats;

#[cfg(test)]
mod tests {
    mod test_component_kind_display;
    mod test_component_license_display;
    mod test_component_new;
    mod test_component_is_open_source;
    mod test_component_metadata;
    mod test_sbom_add_component;
    mod test_sbom_find;
    mod test_sbom_proprietary_count;
    mod test_sbom_store;
    mod test_signature_register_verify;
    mod test_signature_verify_mismatch;
    mod test_signature_by_signer;
    mod test_provenance_record;
    mod test_provenance_store;
    mod test_provenance_by_kind;
    mod test_policy_deny_license;
    mod test_policy_require_signature;
    mod test_policy_require_provenance;
    mod test_policy_allowed_suppliers;
    mod test_audit_log_record;
    mod test_audit_log_for_tenant;
    mod test_audit_log_for_component;
    mod test_audit_log_failures;
    mod test_report_generate;
    mod test_report_sign_rate;
    mod test_stats_from;
    mod test_stats_oss_rate;
    mod test_builder;
    mod test_query_by_kind;
    mod test_query_open_source_only;
}
