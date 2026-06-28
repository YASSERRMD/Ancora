use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityKind {
    Human,
    Service,
    Device,
    Workload,
}

impl fmt::Display for IdentityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IdentityKind::Human => "HUMAN",
            IdentityKind::Service => "SERVICE",
            IdentityKind::Device => "DEVICE",
            IdentityKind::Workload => "WORKLOAD",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityStatus {
    Active,
    Suspended,
    Revoked,
}

impl fmt::Display for IdentityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IdentityStatus::Active => "ACTIVE",
            IdentityStatus::Suspended => "SUSPENDED",
            IdentityStatus::Revoked => "REVOKED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Identity {
    pub id: String,
    pub tenant_id: String,
    pub kind: IdentityKind,
    pub status: IdentityStatus,
    pub groups: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_tick: u64,
}

impl Identity {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, kind: IdentityKind, tick: u64) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            kind,
            status: IdentityStatus::Active,
            groups: Vec::new(),
            metadata: HashMap::new(),
            created_tick: tick,
        }
    }

    pub fn add_group(&mut self, group: impl Into<String>) { self.groups.push(group.into()); }
    pub fn suspend(&mut self) { self.status = IdentityStatus::Suspended; }
    pub fn revoke(&mut self) { self.status = IdentityStatus::Revoked; }
    pub fn is_active(&self) -> bool { self.status == IdentityStatus::Active }
    pub fn in_group(&self, group: &str) -> bool { self.groups.contains(&group.to_string()) }
    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into()); self
    }
}
