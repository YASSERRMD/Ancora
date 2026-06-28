use crate::incident::Incident;
use crate::runbook::Runbook;
use crate::timeline::IncidentTimeline;

pub struct Postmortem {
    pub incident_id: String,
    pub tenant_id: String,
    pub title: String,
    pub severity_str: String,
    pub duration_ticks: u64,
    pub timeline_event_count: usize,
    pub steps_completed: usize,
    pub steps_total: usize,
    pub root_cause: String,
    pub remediation: String,
    pub tick: u64,
}

impl Postmortem {
    pub fn generate(
        incident: &Incident,
        runbook: Option<&Runbook>,
        timeline: &IncidentTimeline,
        current_tick: u64,
        root_cause: impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        let (steps_completed, steps_total) = runbook
            .map(|r| (r.completed_count(), r.step_count()))
            .unwrap_or((0, 0));
        Self {
            incident_id: incident.id.clone(),
            tenant_id: incident.tenant_id.clone(),
            title: incident.title.clone(),
            severity_str: format!("{}", incident.severity),
            duration_ticks: incident.duration(current_tick),
            timeline_event_count: timeline.for_incident(&incident.id).len(),
            steps_completed,
            steps_total,
            root_cause: root_cause.into(),
            remediation: remediation.into(),
            tick: current_tick,
        }
    }

    pub fn runbook_completion_rate(&self) -> f64 {
        if self.steps_total == 0 { return 0.0; }
        self.steps_completed as f64 / self.steps_total as f64
    }
}
