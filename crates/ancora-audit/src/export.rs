use crate::entry::AuditEntry;

pub fn to_json(entries: &[&AuditEntry]) -> String {
    let parts: Vec<String> = entries.iter().map(|e| {
        format!(
            r#"{{"id":{},"tick":{},"tenant_id":"{}","subject":"{}","operation":"{}","resource":"{}","outcome":"{:?}","severity":"{:?}","checksum":{}}}"#,
            e.id, e.tick, e.tenant_id, e.subject, e.operation, e.resource, e.outcome, e.severity, e.checksum
        )
    }).collect();
    format!("[{}]", parts.join(","))
}

pub fn to_csv(entries: &[&AuditEntry]) -> String {
    let mut out = String::from("id,tick,tenant_id,subject,operation,resource,outcome,severity\n");
    for e in entries {
        out.push_str(&format!(
            "{},{},{},{},{},{},{:?},{:?}\n",
            e.id, e.tick, e.tenant_id, e.subject, e.operation, e.resource, e.outcome, e.severity
        ));
    }
    out
}
