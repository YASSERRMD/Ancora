use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AncoraCluster CRD: defines a full Ancora deployment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncoraCluster {
    pub metadata: ObjectMeta,
    pub spec: AncoraClusterSpec,
    #[serde(default)]
    pub status: AncoraClusterStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncoraClusterSpec {
    pub control_plane_replicas: usize,
    pub worker_replicas: usize,
    pub worker_concurrency: usize,
    pub journal_store: JournalStoreConfig,
    pub autoscaler_enabled: bool,
    pub min_workers: usize,
    pub max_workers: usize,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub secret_refs: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JournalStoreConfig {
    pub backend: String,
    pub connection_secret_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AncoraClusterStatus {
    pub conditions: Vec<Condition>,
    pub observed_generation: i64,
    pub control_plane_ready: bool,
    pub worker_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: String,
    pub status: ConditionStatus,
    pub message: String,
    pub last_transition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionStatus {
    True,
    False,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub name: String,
    pub namespace: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub generation: i64,
    pub finalizers: Vec<String>,
}
