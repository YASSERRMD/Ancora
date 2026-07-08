use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnterpriseAction {
    LicenseIssued,
    LicenseExpired,
    FeatureEnabled,
    FeatureDisabled,
    IncidentOpened,
    IncidentResolved,
    CheckpointRun,
    PostureAssessed,
    ReportGenerated,
}

impl fmt::Display for EnterpriseAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnterpriseAction::LicenseIssued => "LICENSE_ISSUED",
            EnterpriseAction::LicenseExpired => "LICENSE_EXPIRED",
            EnterpriseAction::FeatureEnabled => "FEATURE_ENABLED",
            EnterpriseAction::FeatureDisabled => "FEATURE_DISABLED",
            EnterpriseAction::IncidentOpened => "INCIDENT_OPENED",
            EnterpriseAction::IncidentResolved => "INCIDENT_RESOLVED",
            EnterpriseAction::CheckpointRun => "CHECKPOINT_RUN",
            EnterpriseAction::PostureAssessed => "POSTURE_ASSESSED",
            EnterpriseAction::ReportGenerated => "REPORT_GENERATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct EnterpriseAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub action: EnterpriseAction,
    pub actor: String,
    pub detail: String,
}

impl EnterpriseAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        action: EnterpriseAction,
        actor: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            action,
            actor: actor.into(),
            detail: detail.into(),
        }
    }
}

pub struct EnterpriseAuditLog {
    entries: VecDeque<EnterpriseAuditEntry>,
}

impl EnterpriseAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: EnterpriseAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a EnterpriseAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn by_action<'a>(&'a self, action: &EnterpriseAction) -> Vec<&'a EnterpriseAuditEntry> {
        self.entries
            .iter()
            .filter(|e| &e.action == action)
            .collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &EnterpriseAuditEntry> {
        self.entries.iter()
    }
}
