use crate::entry::{AuditEntry, Outcome, Severity};

#[derive(Default)]
pub struct AuditQuery {
    tenant_id: Option<String>,
    subject: Option<String>,
    operation: Option<String>,
    outcome: Option<Outcome>,
    severity: Option<Severity>,
    tick_from: Option<u64>,
    tick_to: Option<u64>,
}

impl AuditQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tenant(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(id.into());
        self
    }
    pub fn subject(mut self, s: impl Into<String>) -> Self {
        self.subject = Some(s.into());
        self
    }
    pub fn operation(mut self, op: impl Into<String>) -> Self {
        self.operation = Some(op.into());
        self
    }
    pub fn outcome(mut self, o: Outcome) -> Self {
        self.outcome = Some(o);
        self
    }
    pub fn severity(mut self, s: Severity) -> Self {
        self.severity = Some(s);
        self
    }
    pub fn tick_from(mut self, t: u64) -> Self {
        self.tick_from = Some(t);
        self
    }
    pub fn tick_to(mut self, t: u64) -> Self {
        self.tick_to = Some(t);
        self
    }

    pub fn run<'a>(&self, entries: impl Iterator<Item = &'a AuditEntry>) -> Vec<&'a AuditEntry> {
        entries
            .filter(|e| {
                self.tenant_id.as_deref().is_none_or(|t| e.tenant_id == t)
                    && self.subject.as_deref().is_none_or(|s| e.subject == s)
                    && self.operation.as_deref().is_none_or(|op| e.operation == op)
                    && self.outcome.as_ref().is_none_or(|o| &e.outcome == o)
                    && self.severity.as_ref().is_none_or(|s| &e.severity == s)
                    && self.tick_from.is_none_or(|t| e.tick >= t)
                    && self.tick_to.is_none_or(|t| e.tick <= t)
            })
            .collect()
    }
}
