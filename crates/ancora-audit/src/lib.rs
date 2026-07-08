//! Immutable tamper-evident audit log for the Ancora enterprise agent framework.
//!
//! Core types: [`AuditEntry`], [`ImmutableAuditLog`], [`AuditEntryBuilder`], [`AuditStats`].
//! Filtering: [`AuditQuery`] for multi-field queries, log filter methods for single-field queries.
//! Analysis: [`summarize_by_tenant`] for per-tenant [`AuditStats`] aggregation.
//! Retention: [`RetentionPolicy`] identifies entries older than a tick-age threshold.
//! Export: [`to_json`] and [`to_csv`] for reporting and archiving.
pub mod builder;
pub mod display;
pub mod entry;
pub mod error;
pub mod export;
pub mod log;
pub mod query;
pub mod retention;
pub mod stats;
pub mod tenant_summary;

pub use builder::AuditEntryBuilder;
pub use entry::{AuditEntry, Outcome, Severity};
pub use error::AuditError;
pub use export::{to_csv, to_json};
pub use log::ImmutableAuditLog;
pub use query::AuditQuery;
pub use retention::RetentionPolicy;
pub use stats::AuditStats;
pub use tenant_summary::{summarize_by_tenant, TenantSummary};

#[cfg(test)]
mod tests {
    mod test_builder;
    mod test_display;
    mod test_entry_checksum;
    mod test_entry_details;
    mod test_entry_tamper;
    mod test_error;
    mod test_export_csv;
    mod test_export_json;
    mod test_log_append;
    mod test_log_count;
    mod test_log_empty;
    mod test_log_filter_operation;
    mod test_log_filter_subject;
    mod test_log_filter_tenant;
    mod test_log_filter_tick_range;
    mod test_log_get_by_id;
    mod test_log_immutable_ids;
    mod test_log_max_size;
    mod test_log_verify_all;
    mod test_outcome_variants;
    mod test_query_builder;
    mod test_retention_policy;
    mod test_severity_variants;
    mod test_stats_basic;
    mod test_stats_failure_rate;
    mod test_stats_severity;
    mod test_tenant_summary;
}
