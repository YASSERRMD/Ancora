use crate::crd::cluster::AncoraClusterSpec;
use crate::crd::tenant::AncoraTenantSpec;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("invalid spec: {0}")]
    InvalidSpec(String),
}

/// Admission validation for AncoraCluster.
pub fn validate_cluster(spec: &AncoraClusterSpec) -> Result<(), ValidationError> {
    if spec.control_plane_replicas == 0 {
        return Err(ValidationError::InvalidSpec(
            "control_plane_replicas must be > 0".into(),
        ));
    }
    if spec.min_workers > spec.max_workers {
        return Err(ValidationError::InvalidSpec(
            "min_workers must be <= max_workers".into(),
        ));
    }
    if spec.journal_store.backend.is_empty() {
        return Err(ValidationError::InvalidSpec(
            "journal_store.backend must not be empty".into(),
        ));
    }
    Ok(())
}

/// Admission validation for AncoraTenant.
pub fn validate_tenant(spec: &AncoraTenantSpec) -> Result<(), ValidationError> {
    if spec.cluster_ref.is_empty() {
        return Err(ValidationError::InvalidSpec(
            "cluster_ref must not be empty".into(),
        ));
    }
    if spec.max_workers == 0 {
        return Err(ValidationError::InvalidSpec(
            "max_workers must be > 0".into(),
        ));
    }
    Ok(())
}
