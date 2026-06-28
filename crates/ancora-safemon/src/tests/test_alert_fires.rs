use crate::alerting::{AlertChannel, AlertManager, AlertThreshold, InMemoryChannel};
use crate::incident_log::{Incident, IncidentSeverity};

fn make_incident(id: u64, severity: IncidentSeverity, category: &str) -> Incident {
    Incident {
        id,
        severity,
        category: category.to_string(),
        description: "test".to_string(),
        timestamp: id,
        excerpt: "excerpt".to_string(),
        agent_id: None,
    }
}

#[test]
fn alert_fires_for_high_severity() {
    let threshold = AlertThreshold::new(IncidentSeverity::Medium);
    let mut mgr = AlertManager::new(threshold);
    let ch = std::sync::Arc::new(InMemoryChannel::new("test"));

    struct ArcChannel(std::sync::Arc<InMemoryChannel>);
    impl crate::alerting::AlertChannel for ArcChannel {
        fn name(&self) -> &str {
            self.0.name()
        }
        fn send(&self, alert: &crate::alerting::Alert) -> Result<(), String> {
            self.0.send(alert)
        }
    }

    mgr.add_channel(Box::new(ArcChannel(ch.clone())));
    let incident = make_incident(1, IncidentSeverity::High, "pii");
    mgr.process(&incident);

    assert_eq!(ch.count(), 1);
    assert_eq!(mgr.total_fired(), 1);
}

#[test]
fn alert_does_not_fire_below_threshold() {
    let threshold = AlertThreshold::new(IncidentSeverity::High);
    let mut mgr = AlertManager::new(threshold);

    struct NoopChannel;
    impl crate::alerting::AlertChannel for NoopChannel {
        fn name(&self) -> &str { "noop" }
        fn send(&self, _: &crate::alerting::Alert) -> Result<(), String> {
            Err("should not fire".to_string())
        }
    }

    mgr.add_channel(Box::new(NoopChannel));
    let incident = make_incident(2, IncidentSeverity::Low, "hallucination");
    let results = mgr.process(&incident);
    assert!(results.is_empty());
    assert_eq!(mgr.total_fired(), 0);
}

#[test]
fn multiple_channels_receive_alert() {
    let threshold = AlertThreshold::new(IncidentSeverity::Low);
    let mut mgr = AlertManager::new(threshold);
    mgr.add_channel(Box::new(InMemoryChannel::new("ch1")));
    mgr.add_channel(Box::new(InMemoryChannel::new("ch2")));

    let incident = make_incident(3, IncidentSeverity::Medium, "policy");
    let results = mgr.process(&incident);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_ok()));
}

#[test]
fn in_memory_channel_stores_alert_details() {
    let ch = InMemoryChannel::new("audit");
    let alert = crate::alerting::Alert {
        incident_id: 99,
        severity: IncidentSeverity::Critical,
        message: "Critical safety alert".to_string(),
        channel: "audit".to_string(),
    };
    ch.send(&alert).unwrap();
    let stored = ch.alerts();
    assert_eq!(stored.len(), 1);
    assert_eq!(stored[0].incident_id, 99);
    assert_eq!(stored[0].severity, IncidentSeverity::Critical);
}
