//! Key lifecycle management with HSM integration stub, rotation, and audit for Ancora.
//!
//! Core types: [`CryptoKey`], [`KeyAlgorithm`], [`KeyPurpose`], [`KeyStatus`].
//! Store: [`KeyStore`] with multi-version key storage and tenant isolation.
//! Rotation: [`rotate_key`] and [`RotationPolicy`] for automated key rotation.
//! HSM: [`HsmStub`] with [`HsmConfig`] for hardware-backed key generation simulation.
//! Audit: [`KeyAuditLog`] tracking all key lifecycle operations.
//! Expiry: [`ExpiryChecker`] for expired and expiring-soon key detection.
//! Stats: [`KeyStats`] with per-tenant active key counts and algorithm distribution.
//! Builder: [`KeyBuilder`] fluent constructor for [`CryptoKey`].
pub mod audit;
pub mod builder;
pub mod display;
pub mod expiry;
pub mod hsm;
pub mod key;
pub mod lifecycle;
pub mod presets;
pub mod rotation;
pub mod stats;
pub mod store;
pub mod summary;
pub mod validator;

pub use audit::{KeyAuditEntry, KeyAuditLog, KeyOperation};
pub use builder::KeyBuilder;
pub use expiry::ExpiryChecker;
pub use hsm::{HsmBackend, HsmConfig, HsmStub};
pub use key::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus};
pub use lifecycle::{compromise_key, deactivate_key, destroy_key, schedule_key_deletion};
pub use presets::{
    aes256_encryption_key, ed25519_signing_key, ephemeral_key, hmac256_signing_key,
    rsa2048_auth_key,
};
pub use rotation::{rotate_key, RotationPolicy};
pub use stats::{KeyStats, KeyStatusSummary};
pub use store::{KeyStore, KeyStoreError};
pub use validator::{KeyValidator, ValidationIssue};

#[cfg(test)]
mod tests {
    mod test_audit_log_for_key;
    mod test_audit_log_for_tenant;
    mod test_audit_log_record;
    mod test_audit_log_rotations;
    mod test_builder_defaults;
    mod test_builder_full;
    mod test_expiry_expired_keys;
    mod test_expiry_expiring_soon;
    mod test_hsm_backend_display;
    mod test_hsm_generate_key;
    mod test_hsm_hardware_backed;
    mod test_key_algorithm_display;
    mod test_key_compromise;
    mod test_key_deactivate;
    mod test_key_destroy;
    mod test_key_expiry;
    mod test_key_metadata;
    mod test_key_new;
    mod test_key_purpose_display;
    mod test_key_status_display;
    mod test_lifecycle;
    mod test_presets;
    mod test_rotate_key;
    mod test_rotate_key_new_version;
    mod test_rotation_policy_should_rotate;
    mod test_stats_for_tenant;
    mod test_store_create;
    mod test_store_duplicate_error;
    mod test_store_get_active;
    mod test_store_get_version;
    mod test_store_list_tenant_active;
    mod test_store_version_count;
    mod test_summary;
    mod test_validator;
}
