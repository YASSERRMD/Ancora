use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncidentAction {
    Created,
    StatusUpdated,
    Assigned,
    Escalated,
    RunbookStarted,
    RunbookStepDone,
    Resolved,
    PostmortemCreated,
}

impl fmt::Display for IncidentAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IncidentAction::Created => "CREATED",
            IncidentAction::StatusUpdated => "STATUS_UPDATED",
            IncidentAction::Assigned => "ASSIGNED",
            IncidentAction::Escalated => "ESCALATED",
            IncidentAction::RunbookStarted => "RUNBOOK_STARTED",
            IncidentAction::RunbookStepDone => "RUNBOOK_STEP_DONE",
            IncidentAction::Resolved => "RESOLVED",
            IncidentAction::PostmortemCreated => "POSTMORTEM_CREATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct IncidentAuditEntry {
    pub tick: u64,
    pub incident_id: String,
    pub tenant_id: String,
    pub action: IncidentAction,
    pub actor: String,
    pub detail: String,
}

impl IncidentAuditEntry {
    pub fn new(
        tick: u64,
        incident_id: impl Into<String>,
        tenant_id: impl Into<String>,
        action: IncidentAction,
        actor: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            incident_id: incident_id.into(),
            tenant_id: tenant_id.into(),
            action,
            actor: actor.into(),
            detail: detail.into(),
        }
    }
}

pub struct IncidentAuditLog {
    entries: VecDeque<IncidentAuditEntry>,
}

impl Default for IncidentAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl IncidentAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: IncidentAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_incident<'a>(&'a self, incident_id: &str) -> Vec<&'a IncidentAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.incident_id == incident_id)
            .collect()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a IncidentAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &IncidentAuditEntry> {
        self.entries.iter()
    }
}
