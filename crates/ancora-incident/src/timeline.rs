use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimelineEventKind {
    Detected,
    Assigned,
    StatusChanged,
    RunbookStepCompleted,
    EscalationTriggered,
    Note,
    Resolved,
}

impl fmt::Display for TimelineEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TimelineEventKind::Detected => "DETECTED",
            TimelineEventKind::Assigned => "ASSIGNED",
            TimelineEventKind::StatusChanged => "STATUS_CHANGED",
            TimelineEventKind::RunbookStepCompleted => "RUNBOOK_STEP_COMPLETED",
            TimelineEventKind::EscalationTriggered => "ESCALATION_TRIGGERED",
            TimelineEventKind::Note => "NOTE",
            TimelineEventKind::Resolved => "RESOLVED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub incident_id: String,
    pub kind: TimelineEventKind,
    pub author: String,
    pub detail: String,
    pub tick: u64,
}

impl TimelineEvent {
    pub fn new(
        incident_id: impl Into<String>,
        kind: TimelineEventKind,
        author: impl Into<String>,
        detail: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            incident_id: incident_id.into(),
            kind,
            author: author.into(),
            detail: detail.into(),
            tick,
        }
    }
}

pub struct IncidentTimeline {
    events: Vec<TimelineEvent>,
}

impl IncidentTimeline {
    pub fn new() -> Self { Self { events: Vec::new() } }
    pub fn add(&mut self, event: TimelineEvent) { self.events.push(event); }
    pub fn count(&self) -> usize { self.events.len() }
    pub fn for_incident<'a>(&'a self, incident_id: &str) -> Vec<&'a TimelineEvent> {
        self.events.iter().filter(|e| e.incident_id == incident_id).collect()
    }
    pub fn by_kind<'a>(&'a self, kind: &TimelineEventKind) -> Vec<&'a TimelineEvent> {
        self.events.iter().filter(|e| &e.kind == kind).collect()
    }
    pub fn all(&self) -> &[TimelineEvent] { &self.events }
}
