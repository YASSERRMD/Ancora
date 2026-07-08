use crate::audit::AuditEvent;
use serde_json::{json, Value};

/// Export an audit event in a SIEM-compatible format (Common Event Format fields).
pub fn to_siem(event: &AuditEvent) -> Value {
    json!({
        "Version": "0",
        "DeviceVendor": "Ancora",
        "DeviceProduct": "AncoraPlatform",
        "DeviceVersion": "0.6.0",
        "SignatureID": format!("{:?}", event.kind),
        "Name": event.decision,
        "Severity": "5",
        "Extensions": {
            "suser": event.actor,
            "dhost": event.resource,
            "cs1": event.tenant_id,
            "cs1Label": "TenantId",
            "rt": event.timestamp_secs,
            "sig": event.signature
        }
    })
}
