use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncidentStatus {
    Detected,
    Triaged,
    Investigating,
    Mitigating,
    Resolved,
    Closed,
}

impl fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IncidentStatus::Detected => "DETECTED",
            IncidentStatus::Triaged => "TRIAGED",
            IncidentStatus::Investigating => "INVESTIGATING",
            IncidentStatus::Mitigating => "MITIGATING",
            IncidentStatus::Resolved => "RESOLVED",
            IncidentStatus::Closed => "CLOSED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Incident {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub detected_tick: u64,
    pub resolved_tick: Option<u64>,
    pub assignee: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Incident {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        title: impl Into<String>,
        severity: Severity,
        detected_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            title: title.into(),
            severity,
            status: IncidentStatus::Detected,
            detected_tick,
            resolved_tick: None,
            assignee: None,
            metadata: HashMap::new(),
        }
    }

    pub fn assign(&mut self, assignee: impl Into<String>) {
        self.assignee = Some(assignee.into());
    }

    pub fn triage(&mut self) {
        self.status = IncidentStatus::Triaged;
    }
    pub fn investigate(&mut self) {
        self.status = IncidentStatus::Investigating;
    }
    pub fn mitigate(&mut self) {
        self.status = IncidentStatus::Mitigating;
    }

    pub fn resolve(&mut self, tick: u64) {
        self.status = IncidentStatus::Resolved;
        self.resolved_tick = Some(tick);
    }

    pub fn close(&mut self) {
        self.status = IncidentStatus::Closed;
    }

    pub fn is_active(&self) -> bool {
        !matches!(
            self.status,
            IncidentStatus::Resolved | IncidentStatus::Closed
        )
    }

    pub fn duration(&self, current_tick: u64) -> u64 {
        self.resolved_tick
            .unwrap_or(current_tick)
            .saturating_sub(self.detected_tick)
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
