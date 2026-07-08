use crate::entry::AuditEntry;
use std::fmt;

impl fmt::Display for AuditEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[tick={} id={} tenant={} subject={}] {}/{} -> {:?} ({:?})",
            self.tick,
            self.id,
            self.tenant_id,
            self.subject,
            self.operation,
            self.resource,
            self.outcome,
            self.severity
        )
    }
}
