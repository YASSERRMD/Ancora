use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    P1, // total outage
    P2, // major degradation
    P3, // minor degradation
    P4, // low impact
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Mitigated,
    Resolved,
    PostMortem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub opened_at: u64,
    pub resolved_at: Option<u64>,
    pub commander: String,
    pub summary: String,
}

impl Incident {
    pub fn new(id: &str, title: &str, severity: Severity, opened_at: u64, commander: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            severity,
            status: IncidentStatus::Open,
            opened_at,
            resolved_at: None,
            commander: commander.to_string(),
            summary: String::new(),
        }
    }

    pub fn mitigate(&mut self) {
        self.status = IncidentStatus::Mitigated;
    }

    pub fn resolve(&mut self, at: u64) {
        self.status = IncidentStatus::Resolved;
        self.resolved_at = Some(at);
    }

    pub fn ttm_secs(&self) -> Option<u64> {
        self.resolved_at.map(|r| r.saturating_sub(self.opened_at))
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self.status, IncidentStatus::Resolved | IncidentStatus::PostMortem)
    }
}
