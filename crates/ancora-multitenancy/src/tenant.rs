use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque tenant identifier. Propagated on every run, journal entry, and cost record.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub String);

impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_raw(s: &str) -> Self {
        Self(s.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}

/// Tenant lifecycle state.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantState {
    Active,
    Suspended,
    Deleted,
}

/// Per-tenant configuration overrides.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantConfig {
    /// LLM providers this tenant may call.
    pub provider_allowlist: Vec<String>,
    /// Data residency region tag (e.g. "uae-north", "eu-west").
    pub residency_region: Option<String>,
    /// Maximum concurrent workers this tenant may use.
    pub max_workers: u32,
}

impl Default for TenantConfig {
    fn default() -> Self {
        Self {
            provider_allowlist: vec![],
            residency_region: None,
            max_workers: 5,
        }
    }
}

/// Full tenant record managed by `TenantRegistry`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub state: TenantState,
    pub config: TenantConfig,
}

impl Tenant {
    pub fn new(name: impl Into<String>, config: TenantConfig) -> Self {
        Self {
            id: TenantId::new(),
            name: name.into(),
            state: TenantState::Active,
            config,
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == TenantState::Active
    }
}
