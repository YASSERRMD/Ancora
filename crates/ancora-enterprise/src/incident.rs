use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IncidentSeverity::Low => "LOW",
            IncidentSeverity::Medium => "MEDIUM",
            IncidentSeverity::High => "HIGH",
            IncidentSeverity::Critical => "CRITICAL",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncidentStatus {
    Open,
    Investigating,
    Contained,
    Resolved,
    Closed,
}

impl fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IncidentStatus::Open => "OPEN",
            IncidentStatus::Investigating => "INVESTIGATING",
            IncidentStatus::Contained => "CONTAINED",
            IncidentStatus::Resolved => "RESOLVED",
            IncidentStatus::Closed => "CLOSED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct EnterpriseIncident {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub source_domain: String,
    pub opened_tick: u64,
    pub resolved_tick: Option<u64>,
    pub assignee: Option<String>,
}

impl EnterpriseIncident {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        title: impl Into<String>,
        severity: IncidentSeverity,
        source_domain: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            title: title.into(),
            severity,
            status: IncidentStatus::Open,
            source_domain: source_domain.into(),
            opened_tick: tick,
            resolved_tick: None,
            assignee: None,
        }
    }

    pub fn with_assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    pub fn investigate(&mut self) { self.status = IncidentStatus::Investigating; }
    pub fn contain(&mut self) { self.status = IncidentStatus::Contained; }
    pub fn resolve(&mut self, tick: u64) {
        self.status = IncidentStatus::Resolved;
        self.resolved_tick = Some(tick);
    }
    pub fn close(&mut self) { self.status = IncidentStatus::Closed; }

    pub fn is_open(&self) -> bool { self.status == IncidentStatus::Open }
    pub fn is_resolved(&self) -> bool { self.status == IncidentStatus::Resolved || self.status == IncidentStatus::Closed }
    pub fn is_critical(&self) -> bool { self.severity == IncidentSeverity::Critical }

    pub fn time_to_resolve(&self, current_tick: u64) -> u64 {
        let end = self.resolved_tick.unwrap_or(current_tick);
        end.saturating_sub(self.opened_tick)
    }
}

pub struct IncidentLog {
    incidents: Vec<EnterpriseIncident>,
}

impl IncidentLog {
    pub fn new() -> Self { Self { incidents: Vec::new() } }
    pub fn record(&mut self, i: EnterpriseIncident) { self.incidents.push(i); }
    pub fn count(&self) -> usize { self.incidents.len() }
    pub fn open(&self) -> Vec<&EnterpriseIncident> { self.incidents.iter().filter(|i| i.is_open()).collect() }
    pub fn critical(&self) -> Vec<&EnterpriseIncident> { self.incidents.iter().filter(|i| i.is_critical()).collect() }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a EnterpriseIncident> {
        self.incidents.iter().filter(|i| i.tenant_id == tenant_id).collect()
    }
    pub fn resolved(&self) -> Vec<&EnterpriseIncident> { self.incidents.iter().filter(|i| i.is_resolved()).collect() }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut EnterpriseIncident> {
        self.incidents.iter_mut().find(|i| i.id == id)
    }
    pub fn all(&self) -> impl Iterator<Item = &EnterpriseIncident> { self.incidents.iter() }
}
