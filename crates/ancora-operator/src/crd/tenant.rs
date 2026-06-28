use serde::{Deserialize, Serialize};
use crate::crd::cluster::ObjectMeta;

/// AncoraTenant CRD: defines an isolated tenant within a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncoraTenant {
    pub metadata: ObjectMeta,
    pub spec: AncoraTenantSpec,
    #[serde(default)]
    pub status: AncoraTenantStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncoraTenantSpec {
    pub cluster_ref: String,
    pub max_workers: usize,
    pub provider_allowlist: Vec<String>,
    pub residency_region: Option<String>,
    pub admin_role_binding: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AncoraTenantStatus {
    pub provisioned: bool,
    pub namespace: Option<String>,
    pub conditions: Vec<crate::crd::cluster::Condition>,
}
