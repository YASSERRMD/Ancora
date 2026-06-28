//! Safety monitoring for detecting policy violations in agent output.

/// Severity level of a safety event.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// A safety policy violation event.
#[derive(Debug, Clone)]
pub struct SafetyEvent {
    pub severity: Severity,
    pub policy_id: String,
    pub description: String,
    pub agent_id: String,
}

impl SafetyEvent {
    pub fn new(
        severity: Severity,
        policy_id: impl Into<String>,
        description: impl Into<String>,
        agent_id: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            policy_id: policy_id.into(),
            description: description.into(),
            agent_id: agent_id.into(),
        }
    }
}

/// Collects and filters safety events.
#[derive(Debug, Default)]
pub struct SafetyMonitor {
    events: Vec<SafetyEvent>,
}

impl SafetyMonitor {
    pub fn record(&mut self, event: SafetyEvent) {
        self.events.push(event);
    }

    /// Returns events at or above the given severity.
    pub fn events_at_or_above(&self, min_severity: &Severity) -> Vec<&SafetyEvent> {
        self.events
            .iter()
            .filter(|e| &e.severity >= min_severity)
            .collect()
    }

    pub fn critical_count(&self) -> usize {
        self.events_at_or_above(&Severity::Critical).len()
    }

    pub fn has_critical(&self) -> bool {
        self.critical_count() > 0
    }
}
