//! Data classification with sensitivity labels, enforcement, and audit for Ancora.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use ancora_dataclass::*;
//! let policy = ClassificationPolicy::strict("tenant-1");
//! let record = DataRecordBuilder::new("r1", "tenant-1", "SSN")
//!     .level(SensitivityLevel::Restricted)
//!     .category(DataCategory::Pii)
//!     .tag("gdpr")
//!     .build();
//! let decision = ClassificationEnforcer::check_write(&policy, &record);
//! assert!(ClassificationEnforcer::is_allowed(&decision));
//! ```
//!
//! Core types: [`SensitivityLevel`], [`DataCategory`], [`DataRecord`].
//! Policy: [`ClassificationPolicy`] with max allowed level and write controls.
//! Enforcement: [`ClassificationEnforcer`] for read/write decisions.
//! Registry: [`DataRegistry`] for multi-tenant record storage.
//! Audit: [`ClassificationAuditLog`] with per-tenant and per-record filtering.
//! Builder: [`DataRecordBuilder`] fluent constructor.
//! Query: [`DataQuery`] with level, category, tag, and tenant filters.
//! Stats: [`DataClassStats`] with level distribution.
//! Downgrade: [`DowngradePolicy`] with minimum level floor.
//! Redaction: [`RedactionConfig`] for masking values at export time.
//! Export: [`to_csv`] and [`to_json`] for records collections.
pub mod audit;
pub mod builder;
pub mod downgrade;
pub mod enforcer;
pub mod export;
pub mod label;
pub mod policy;
pub mod query;
pub mod record;
pub mod redact;
pub mod registry;
pub mod stats;

pub use audit::{AccessKind, ClassificationAuditEntry, ClassificationAuditLog};
pub use builder::DataRecordBuilder;
pub use downgrade::{DowngradePolicy, DowngradeResult};
pub use enforcer::{ClassificationEnforcer, EnforcementDecision};
pub use export::{to_csv, to_json};
pub use label::{DataCategory, SensitivityLevel};
pub use policy::{ClassificationPolicy, PolicyStore};
pub use query::DataQuery;
pub use record::DataRecord;
pub use redact::RedactionConfig;
pub use registry::{DataRegistry, RegistryError};
pub use stats::DataClassStats;

#[cfg(test)]
mod tests {
    mod test_audit_allowed_for_tenant;
    mod test_audit_denied_for_tenant;
    mod test_audit_for_record;
    mod test_audit_record;
    mod test_builder_defaults;
    mod test_builder_full;
    mod test_category_display;
    mod test_downgrade_already_at_level;
    mod test_downgrade_denied_below_minimum;
    mod test_downgrade_policy_apply;
    mod test_enforcer_is_allowed;
    mod test_enforcer_read_allow;
    mod test_enforcer_read_deny_clearance;
    mod test_enforcer_write_allow;
    mod test_enforcer_write_deny_level;
    mod test_enforcer_write_deny_no_tag;
    mod test_export_csv;
    mod test_export_json;
    mod test_level_display;
    mod test_level_numeric;
    mod test_level_order;
    mod test_policy_new;
    mod test_policy_permissive;
    mod test_policy_store;
    mod test_policy_strict;
    mod test_query_by_category;
    mod test_query_by_level;
    mod test_query_by_tag;
    mod test_query_min_level;
    mod test_record_level_checks;
    mod test_record_metadata;
    mod test_record_new;
    mod test_record_tags;
    mod test_redact_config;
    mod test_registry_at_or_above;
    mod test_registry_by_tenant;
    mod test_registry_get;
    mod test_registry_insert;
    mod test_registry_remove;
    mod test_stats_count_at_level;
    mod test_stats_from_records;
}
