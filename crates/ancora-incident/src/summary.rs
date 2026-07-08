use crate::incident::{Incident, IncidentStatus, Severity};

pub struct IncidentSummary {
    pub tenant_id: String,
    pub total: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub active_count: usize,
    pub unassigned_count: usize,
    pub resolved_count: usize,
}

impl IncidentSummary {
    pub fn generate(incidents: &[&Incident], tenant_id: &str) -> Self {
        let tenant: Vec<&&Incident> = incidents
            .iter()
            .filter(|i| i.tenant_id == tenant_id)
            .collect();
        let total = tenant.len();
        let critical_count = tenant
            .iter()
            .filter(|i| i.severity == Severity::Critical)
            .count();
        let high_count = tenant
            .iter()
            .filter(|i| i.severity == Severity::High)
            .count();
        let active_count = tenant.iter().filter(|i| i.is_active()).count();
        let unassigned_count = tenant.iter().filter(|i| i.assignee.is_none()).count();
        let resolved_count = tenant
            .iter()
            .filter(|i| matches!(i.status, IncidentStatus::Resolved | IncidentStatus::Closed))
            .count();
        Self {
            tenant_id: tenant_id.to_string(),
            total,
            critical_count,
            high_count,
            active_count,
            unassigned_count,
            resolved_count,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.critical_count == 0 && self.unassigned_count == 0
    }
}
