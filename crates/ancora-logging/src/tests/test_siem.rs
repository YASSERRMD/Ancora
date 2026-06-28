#[cfg(test)]
mod tests {
    use crate::{
        audit::{AuditEvent, AuditEventKind},
        siem::to_siem,
    };

    const KEY: &[u8] = b"test-signing-key";

    #[test]
    fn siem_export_parses() {
        let e = AuditEvent::new(1000, AuditEventKind::PolicyDecision, "t1", "admin", "res", "allow", KEY);
        let siem = to_siem(&e);
        assert_eq!(siem["DeviceVendor"], "Ancora");
        assert_eq!(siem["Extensions"]["suser"], "admin");
        assert_eq!(siem["Extensions"]["cs1"], "t1");
    }

    #[test]
    fn siem_export_has_signature() {
        let e = AuditEvent::new(1000, AuditEventKind::AdminAction, "t1", "admin", "res", "allow", KEY);
        let siem = to_siem(&e);
        assert!(!siem["Extensions"]["sig"].as_str().unwrap_or("").is_empty());
    }
}
