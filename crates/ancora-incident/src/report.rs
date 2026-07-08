use crate::audit::IncidentAuditLog;
use crate::escalation::EscalationRecord;
use crate::incident::Incident;
use crate::runbook::Runbook;
use crate::timeline::IncidentTimeline;

pub struct IncidentReport {
    pub incident_id: String,
    pub title: String,
    pub severity_str: String,
    pub status_str: String,
    pub timeline_events: usize,
    pub runbook_steps_total: usize,
    pub runbook_steps_done: usize,
    pub audit_entries: usize,
    pub escalation_count: usize,
    pub duration_ticks: u64,
}

impl IncidentReport {
    pub fn generate(
        incident: &Incident,
        runbook: Option<&Runbook>,
        timeline: &IncidentTimeline,
        audit: &IncidentAuditLog,
        escalations: &[EscalationRecord],
        current_tick: u64,
    ) -> Self {
        let (steps_total, steps_done) = runbook
            .map(|r| (r.step_count(), r.completed_count()))
            .unwrap_or((0, 0));
        Self {
            incident_id: incident.id.clone(),
            title: incident.title.clone(),
            severity_str: format!("{}", incident.severity),
            status_str: format!("{}", incident.status),
            timeline_events: timeline.for_incident(&incident.id).len(),
            runbook_steps_total: steps_total,
            runbook_steps_done: steps_done,
            audit_entries: audit.for_incident(&incident.id).len(),
            escalation_count: escalations
                .iter()
                .filter(|e| e.incident_id == incident.id)
                .count(),
            duration_ticks: incident.duration(current_tick),
        }
    }

    pub fn runbook_progress(&self) -> f64 {
        if self.runbook_steps_total == 0 {
            return 0.0;
        }
        self.runbook_steps_done as f64 / self.runbook_steps_total as f64
    }
}
