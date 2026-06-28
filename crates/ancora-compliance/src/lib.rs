//! Compliance reporting for SOC 2, FedRAMP, and ISO 27001 evidence collection for Ancora.
//!
//! Core types: [`Framework`], [`ControlId`], [`ComplianceControl`], [`EvidenceItem`].
//! Registry: [`ControlRegistry`] with per-framework and per-status queries.
//! Report: [`ComplianceReport`] with compliance rate and full-compliance check.
//! Gap analysis: [`GapAnalyzer`] returning [`GapItem`] for non-compliant controls.
//! Presets: [`presets`] module with SOC 2, ISO 27001, and FedRAMP control sets.
//! Audit: [`ComplianceAuditLog`] tracking status transitions.
//! Stats: [`ComplianceStats`] with per-framework aggregates.
//! Export: [`report_to_csv`], [`controls_to_csv`] for reporting.
pub mod audit;
pub mod control;
pub mod evidence;
pub mod export;
pub mod framework;
pub mod gap;
pub mod presets;
pub mod registry;
pub mod report;
pub mod stats;

pub use audit::{AssessmentRecord, ComplianceAuditLog};
pub use control::{ComplianceControl, ControlStatus};
pub use evidence::{EvidenceItem, EvidenceKind, EvidenceStore};
pub use export::{controls_to_csv, report_to_csv};
pub use framework::{ControlId, Framework};
pub use gap::{GapAnalyzer, GapItem};
pub use registry::ControlRegistry;
pub use report::ComplianceReport;
pub use stats::ComplianceStats;

#[cfg(test)]
mod tests {
    mod test_framework_display;
    mod test_control_id;
    mod test_control_new;
    mod test_control_set_status;
    mod test_control_attach_evidence;
    mod test_control_is_compliant;
    mod test_evidence_item_new;
    mod test_evidence_store_insert;
    mod test_evidence_store_for_tenant;
    mod test_registry_register;
    mod test_registry_for_framework;
    mod test_registry_by_status;
    mod test_registry_compliant_count;
    mod test_report_generate;
    mod test_report_compliance_rate;
    mod test_report_is_fully_compliant;
    mod test_gap_analyzer_analyze;
    mod test_gap_analyzer_critical;
    mod test_presets_soc2;
    mod test_presets_iso27001;
    mod test_presets_fedramp;
    mod test_audit_log_record;
    mod test_audit_for_framework;
    mod test_audit_for_control;
    mod test_audit_for_tenant;
    mod test_stats_from_registry;
    mod test_stats_compliance_rate;
    mod test_stats_gap_count;
    mod test_export_report_csv;
    mod test_export_controls_csv;
}
