//! Secret storage with versioning, rotation, TTL expiry, and access logging for Ancora.
//!
//! Core types: [`Secret`], [`SecretVersion`], [`SecretStore`].
//! Path validation: [`validator::validate_path`] -- alphanumeric plus `/.-_`, no spaces.
//! Rotation: [`RotationPolicy`] with configurable max retained versions.
//! Access logging: [`SecretAccessLog`], [`AccessRecord`] for audit trails.
//! Expiry: [`ExpiryChecker`] for TTL-based secret lifecycle.
pub mod access_log;
pub mod display;
pub mod query;
pub mod soft_delete;
pub mod summary;
pub mod error;
pub mod expiry;
pub mod rotation;
pub mod secret;
pub mod store;
pub mod validator;

pub use access_log::{AccessKind, AccessRecord, SecretAccessLog};
pub use error::SecretError;
pub use expiry::ExpiryChecker;
pub use rotation::RotationPolicy;
pub use secret::{Secret, SecretKind, SecretStatus, SecretVersion};
pub use store::SecretStore;
pub use validator::validate_path;
pub use query::SecretQuery;
pub use soft_delete::{is_soft_deleted, soft_delete};
pub use summary::SecretSummary;

#[cfg(test)]
mod tests {
    mod test_secret_new;
    mod test_secret_active_value;
    mod test_secret_version_count;
    mod test_secret_ttl_expiry;
    mod test_secret_kind_variants;
    mod test_store_create;
    mod test_store_read;
    mod test_store_write_version;
    mod test_store_delete;
    mod test_store_list_tenant;
    mod test_validator_valid_paths;
    mod test_validator_invalid_paths;
    mod test_rotation_rotate;
    mod test_rotation_max_versions;
    mod test_access_log_record;
    mod test_access_log_reads_for;
    mod test_access_log_all_for_tenant;
    mod test_expiry_not_expired;
    mod test_expiry_is_expired;
    mod test_expiry_expired_paths;
    mod test_store_duplicate;
    mod test_error_display;
    mod test_secret_version_metadata;
    mod test_rotation_versions_retained;
    mod test_store_count;
    mod test_access_log_kind_variants;
    mod test_secret_status_variants;
    mod test_validator_empty_path;
    mod test_validator_leading_slash;
    mod test_expiry_active_paths;
    mod test_summary;
    mod test_display;
    mod test_query;
    mod test_soft_delete;
}
