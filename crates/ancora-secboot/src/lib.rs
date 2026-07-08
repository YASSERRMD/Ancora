//! Secure boot process integrity verification for Ancora.
//!
//! Core types: [`Measurement`], [`BootChain`], [`BootPolicy`], [`AttestationRecord`].
//! Evaluation: [`IntegrityEvaluator`] with first-match-wins policy decisions.
//! Sealing: [`SealingStore`] binding secrets to boot state digests.
//! Audit: [`BootAuditLog`] tracking all boot lifecycle events.
//! Stats: [`BootStats`] for trust rate metrics.
//! Report: [`IntegrityReport`] aggregating chain, attestation, and decision.
pub mod attestation;
pub mod audit;
pub mod builder;
pub mod chain;
pub mod evaluator;
pub mod measurement;
pub mod policy;
pub mod presets;
pub mod query;
pub mod report;
pub mod seal;
pub mod stats;
pub mod validator;

pub use attestation::{AttestationLog, AttestationRecord, AttestationStatus};
pub use audit::{BootAuditEntry, BootAuditLog, BootEvent};
pub use builder::MeasurementBuilder;
pub use chain::{BootChain, ChainStatus};
pub use evaluator::{IntegrityDecision, IntegrityEvaluator};
pub use measurement::{Measurement, MeasurementKind};
pub use policy::{BootPolicy, PolicyEffect, PolicyStore};
pub use presets::{kernel_only_policy, permissive_boot_policy, strict_boot_policy};
pub use query::MeasurementQuery;
pub use report::IntegrityReport;
pub use seal::{SealResult, SealedBlob, SealingStore, UnsealResult};
pub use stats::BootStats;
pub use validator::{ChainIssue, ChainValidator};

#[cfg(test)]
mod tests {
    mod test_attestation_log_for_node;
    mod test_attestation_log_for_tenant;
    mod test_attestation_record;
    mod test_attestation_trusted;
    mod test_audit_log_failures;
    mod test_audit_log_for_tenant;
    mod test_audit_log_record;
    mod test_builder;
    mod test_chain_add_step;
    mod test_chain_find_by_id;
    mod test_chain_present_kinds;
    mod test_chain_status;
    mod test_evaluator_empty_chain;
    mod test_evaluator_fail_bad_digest;
    mod test_evaluator_fail_missing_kind;
    mod test_evaluator_pass;
    mod test_measurement_kind_display;
    mod test_measurement_matches_digest;
    mod test_measurement_metadata;
    mod test_measurement_new;
    mod test_policy_allow_digest;
    mod test_policy_deny_unknown;
    mod test_policy_new;
    mod test_policy_require_kind;
    mod test_policy_store;
    mod test_presets;
    mod test_query;
    mod test_report_generate;
    mod test_seal_already_sealed;
    mod test_seal_and_unseal;
    mod test_seal_policy_mismatch;
    mod test_stats_trust_rate;
    mod test_validator;
}
