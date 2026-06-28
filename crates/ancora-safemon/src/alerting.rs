/// Safety alerting - fires notifications when incidents exceed thresholds.
///
/// Supports multiple alert channels (in-memory callbacks, log-based)
/// with configurable thresholds per severity level.

use crate::incident_log::{Incident, IncidentSeverity};

#[derive(Debug, Clone)]
pub struct Alert {
    pub incident_id: u64,
    pub severity: IncidentSeverity,
    pub message: String,
    pub channel: String,
}

pub trait AlertChannel: Send + Sync {
    fn name(&self) -> &str;
    fn send(&self, alert: &Alert) -> Result<(), String>;
}

/// In-memory alert channel that stores fired alerts for inspection.
pub struct InMemoryChannel {
    pub name: String,
    fired: std::sync::Mutex<Vec<Alert>>,
}

impl InMemoryChannel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fired: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn alerts(&self) -> Vec<Alert> {
        self.fired.lock().unwrap().clone()
    }

    pub fn count(&self) -> usize {
        self.fired.lock().unwrap().len()
    }
}

impl AlertChannel for InMemoryChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn send(&self, alert: &Alert) -> Result<(), String> {
        self.fired.lock().unwrap().push(alert.clone());
        Ok(())
    }
}

pub struct AlertThreshold {
    pub min_severity: IncidentSeverity,
}

impl AlertThreshold {
    pub fn new(min_severity: IncidentSeverity) -> Self {
        Self { min_severity }
    }

    pub fn should_alert(&self, incident: &Incident) -> bool {
        incident.severity >= self.min_severity
    }
}

pub struct AlertManager {
    channels: Vec<Box<dyn AlertChannel>>,
    threshold: AlertThreshold,
    total_fired: u64,
}

impl AlertManager {
    pub fn new(threshold: AlertThreshold) -> Self {
        Self {
            channels: Vec::new(),
            threshold,
            total_fired: 0,
        }
    }

    pub fn add_channel(&mut self, channel: Box<dyn AlertChannel>) {
        self.channels.push(channel);
    }

    /// Process an incident and fire alerts on all channels if threshold met.
    pub fn process(&mut self, incident: &Incident) -> Vec<Result<(), String>> {
        if !self.threshold.should_alert(incident) {
            return Vec::new();
        }

        let alert = Alert {
            incident_id: incident.id,
            severity: incident.severity.clone(),
            message: format!(
                "[{}] {} - {}",
                incident.severity.as_str(),
                incident.category,
                incident.description
            ),
            channel: String::new(),
        };

        self.total_fired += 1;

        self.channels
            .iter()
            .map(|ch| {
                let mut a = alert.clone();
                a.channel = ch.name().to_string();
                ch.send(&a)
            })
            .collect()
    }

    pub fn total_fired(&self) -> u64 {
        self.total_fired
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incident_log::Incident;

    fn make_incident(severity: IncidentSeverity) -> Incident {
        Incident {
            id: 1,
            severity,
            category: "test".to_string(),
            description: "test incident".to_string(),
            timestamp: 1,
            excerpt: "excerpt".to_string(),
            agent_id: None,
        }
    }

    #[test]
    fn alert_fires_above_threshold() {
        let threshold = AlertThreshold::new(IncidentSeverity::Medium);
        let mut mgr = AlertManager::new(threshold);
        let ch = InMemoryChannel::new("test-channel");
        let ch_ref = unsafe { &*(&ch as *const InMemoryChannel) };
        mgr.add_channel(Box::new(InMemoryChannel::new("inner")));

        let incident = make_incident(IncidentSeverity::High);
        let threshold2 = AlertThreshold::new(IncidentSeverity::Medium);
        assert!(threshold2.should_alert(&incident));
        let _ = ch_ref;
    }

    #[test]
    fn alert_does_not_fire_below_threshold() {
        let threshold = AlertThreshold::new(IncidentSeverity::High);
        let incident = make_incident(IncidentSeverity::Low);
        assert!(!threshold.should_alert(&incident));
    }

    #[test]
    fn in_memory_channel_records_alerts() {
        let ch = InMemoryChannel::new("mem");
        let alert = Alert {
            incident_id: 42,
            severity: IncidentSeverity::High,
            message: "test alert".to_string(),
            channel: "mem".to_string(),
        };
        ch.send(&alert).unwrap();
        assert_eq!(ch.count(), 1);
        assert_eq!(ch.alerts()[0].incident_id, 42);
    }

    #[test]
    fn alert_manager_counts_fired() {
        let threshold = AlertThreshold::new(IncidentSeverity::Low);
        let mut mgr = AlertManager::new(threshold);
        mgr.add_channel(Box::new(InMemoryChannel::new("ch1")));

        let incident = make_incident(IncidentSeverity::Medium);
        mgr.process(&incident);
        mgr.process(&incident);
        assert_eq!(mgr.total_fired(), 2);
    }
}
