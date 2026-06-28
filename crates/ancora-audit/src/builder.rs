use crate::entry::{AuditEntry, Outcome, Severity};

pub struct AuditEntryBuilder {
    tick: u64,
    tenant_id: String,
    subject: String,
    operation: String,
    resource: String,
    outcome: Outcome,
    severity: Severity,
    details: Vec<(String, String)>,
}

impl AuditEntryBuilder {
    pub fn new(tick: u64, tenant_id: impl Into<String>, subject: impl Into<String>) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            subject: subject.into(),
            operation: String::new(),
            resource: String::new(),
            outcome: Outcome::Success,
            severity: Severity::Info,
            details: Vec::new(),
        }
    }

    pub fn operation(mut self, op: impl Into<String>) -> Self { self.operation = op.into(); self }
    pub fn resource(mut self, r: impl Into<String>) -> Self { self.resource = r.into(); self }
    pub fn outcome(mut self, o: Outcome) -> Self { self.outcome = o; self }
    pub fn severity(mut self, s: Severity) -> Self { self.severity = s; self }
    pub fn detail(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.details.push((k.into(), v.into())); self
    }

    pub fn build(self) -> AuditEntry {
        let mut entry = AuditEntry::new(
            0, self.tick, self.tenant_id, self.subject, self.operation, self.resource,
            self.outcome, self.severity,
        );
        for (k, v) in self.details { entry = entry.with_detail(k, v); }
        entry
    }
}
