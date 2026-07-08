use crate::incident::{Incident, IncidentStatus};
use std::collections::HashMap;

pub struct IncidentStats {
    pub tenant_id: String,
    pub total: usize,
    pub active: usize,
    pub resolved: usize,
    pub by_severity: HashMap<String, usize>,
    pub mean_duration: f64,
}

impl IncidentStats {
    pub fn for_tenant(incidents: &[&Incident], tenant_id: &str, current_tick: u64) -> Self {
        let tenant_incidents: Vec<&&Incident> = incidents
            .iter()
            .filter(|i| i.tenant_id == tenant_id)
            .collect();
        let total = tenant_incidents.len();
        let active = tenant_incidents.iter().filter(|i| i.is_active()).count();
        let resolved = tenant_incidents
            .iter()
            .filter(|i| matches!(i.status, IncidentStatus::Resolved | IncidentStatus::Closed))
            .count();
        let mut by_severity = HashMap::new();
        let mut total_duration = 0u64;
        for i in &tenant_incidents {
            *by_severity.entry(format!("{}", i.severity)).or_insert(0) += 1;
            total_duration += i.duration(current_tick);
        }
        let mean_duration = if total == 0 {
            0.0
        } else {
            total_duration as f64 / total as f64
        };
        Self {
            tenant_id: tenant_id.to_string(),
            total,
            active,
            resolved,
            by_severity,
            mean_duration,
        }
    }
}
