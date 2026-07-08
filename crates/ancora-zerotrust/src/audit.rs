use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZtAction {
    AccessGranted,
    AccessDenied,
    MfaRequired,
    SessionCreated,
    SessionRevoked,
    DevicePostureChecked,
    PolicyEvaluated,
}

impl fmt::Display for ZtAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ZtAction::AccessGranted => "ACCESS_GRANTED",
            ZtAction::AccessDenied => "ACCESS_DENIED",
            ZtAction::MfaRequired => "MFA_REQUIRED",
            ZtAction::SessionCreated => "SESSION_CREATED",
            ZtAction::SessionRevoked => "SESSION_REVOKED",
            ZtAction::DevicePostureChecked => "DEVICE_POSTURE_CHECKED",
            ZtAction::PolicyEvaluated => "POLICY_EVALUATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ZtAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub identity_id: String,
    pub resource: String,
    pub action: ZtAction,
    pub success: bool,
    pub detail: String,
}

impl ZtAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        identity_id: impl Into<String>,
        resource: impl Into<String>,
        action: ZtAction,
        success: bool,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            identity_id: identity_id.into(),
            resource: resource.into(),
            action,
            success,
            detail: detail.into(),
        }
    }
}

pub struct ZtAuditLog {
    entries: VecDeque<ZtAuditEntry>,
}

impl ZtAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: ZtAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a ZtAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn for_identity<'a>(&'a self, identity_id: &str) -> Vec<&'a ZtAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.identity_id == identity_id)
            .collect()
    }
    pub fn denied<'a>(&'a self) -> Vec<&'a ZtAuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &ZtAuditEntry> {
        self.entries.iter()
    }
}
