#[cfg(test)]
mod tests {
    use crate::audit::{AuditChannel, AuditEvent, AuditEventKind};

    const KEY: &[u8] = b"test-signing-key";

    fn make_event(kind: AuditEventKind, tenant: &str) -> AuditEvent {
        AuditEvent::new(
            1000,
            kind,
            tenant,
            "admin@ancora.dev",
            "resource-1",
            "allowed",
            KEY,
        )
    }

    #[test]
    fn audit_channel_is_append_only() {
        let mut ch = AuditChannel::new();
        ch.append(make_event(AuditEventKind::PolicyDecision, "t1"));
        ch.append(make_event(AuditEventKind::AdminAction, "t1"));
        assert_eq!(ch.count(), 2);
    }

    #[test]
    fn policy_decision_audited() {
        let mut ch = AuditChannel::new();
        ch.append(make_event(AuditEventKind::PolicyDecision, "t1"));
        let found = ch.query("t1", &AuditEventKind::PolicyDecision);
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn admin_action_audited() {
        let mut ch = AuditChannel::new();
        ch.append(make_event(AuditEventKind::AdminAction, "t1"));
        let found = ch.query("t1", &AuditEventKind::AdminAction);
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn audit_signature_verifies() {
        let e = make_event(AuditEventKind::SecretRotated, "t1");
        assert!(e.verify(KEY));
    }

    #[test]
    fn tampered_event_fails_verification() {
        let mut e = make_event(AuditEventKind::AccessGranted, "t1");
        e.decision = "denied".into();
        assert!(!e.verify(KEY));
    }

    #[test]
    fn query_filters_by_tenant() {
        let mut ch = AuditChannel::new();
        ch.append(make_event(AuditEventKind::PolicyDecision, "t1"));
        ch.append(make_event(AuditEventKind::PolicyDecision, "t2"));
        assert_eq!(ch.query("t1", &AuditEventKind::PolicyDecision).len(), 1);
        assert_eq!(ch.query("t2", &AuditEventKind::PolicyDecision).len(), 1);
    }

    #[test]
    fn audit_query_api_returns_latest() {
        let mut ch = AuditChannel::new();
        ch.append(make_event(AuditEventKind::PolicyDecision, "t1"));
        ch.append(make_event(AuditEventKind::PolicyDecision, "t2"));
        let latest = ch.latest(&AuditEventKind::PolicyDecision);
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().tenant_id, "t2");
    }
}
