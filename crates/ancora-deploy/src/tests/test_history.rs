#[cfg(test)]
mod tests {
    use crate::{DeployEvent, DeployHistory, Version};

    #[test]
    fn deploy_history_recorded() {
        let mut h = DeployHistory::new();
        h.record(DeployEvent::BlueGreenSwitch {
            from: Version::new(1, 0, 0),
            to: Version::new(2, 0, 0),
            duration_ms: 5,
        });
        h.record(DeployEvent::CanaryStarted {
            version: Version::new(3, 0, 0),
            pct: 0.1,
        });
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn rollback_event_stored_in_history() {
        let mut h = DeployHistory::new();
        h.record(DeployEvent::CanaryRolledBack {
            reason: "error rate exceeded".into(),
        });
        assert!(!h.is_empty());
        assert!(matches!(
            h.events()[0],
            DeployEvent::CanaryRolledBack { .. }
        ));
    }
}
