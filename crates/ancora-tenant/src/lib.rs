//! Per-tenant resource quotas, namespace isolation, and lifecycle for Ancora.
//!
//! Core types: [`Tenant`], [`TenantRegistry`], [`ResourceQuota`], [`ResourceUsage`].
//! Admission control: [`AdmissionController`] enforces quotas before resource allocation.
//! Namespace isolation: [`Namespace`] provides scoped key-value storage per tenant.
//! Cross-tenant safety: [`IsolationChecker`] rejects cross-tenant resource access.
//! Reporting: [`TenantSummary`], [`QuotaSummary`] for utilization dashboards.
pub mod admission;
pub mod error;
pub mod isolation;
pub mod namespace;
pub mod quota;
pub mod registry;
pub mod summary;
pub mod tenant;

pub use admission::{AdmissionController, AdmissionDecision};
pub use error::TenantError;
pub use isolation::{IsolationChecker, IsolationResult};
pub use namespace::Namespace;
pub use quota::{ResourceQuota, ResourceUsage};
pub use registry::{RegistryError, TenantRegistry};
pub use summary::{QuotaSummary, TenantSummary};
pub use tenant::{Tenant, TenantStatus};

#[cfg(test)]
mod tests {
    mod test_tenant_lifecycle;
    mod test_tenant_status;
    mod test_tenant_metadata;
    mod test_quota_standard;
    mod test_quota_restricted;
    mod test_quota_unlimited;
    mod test_admission_agents;
    mod test_admission_tasks;
    mod test_admission_memory;
    mod test_admission_secrets;
    mod test_admission_log_entries;
    mod test_namespace_set_get;
    mod test_namespace_remove;
    mod test_namespace_isolation;
    mod test_namespace_scoped_key;
    mod test_registry_register;
    mod test_registry_duplicate;
    mod test_registry_active_list;
    mod test_registry_suspend;
    mod test_registry_require_active;
    mod test_isolation_same_tenant;
    mod test_isolation_cross_tenant;
    mod test_summary_quota_utilization;
    mod test_error_display;
    mod test_registry_not_found;
    mod test_namespace_count;
    mod test_registry_usage;
    mod test_tenant_activate_after_suspend;
    mod test_admission_all_pass;
    mod test_quota_fields;
}
