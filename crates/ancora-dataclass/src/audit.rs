use crate::enforcer::EnforcementDecision;
use crate::label::SensitivityLevel;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessKind {
    Read,
    Write,
    Delete,
}

#[derive(Debug, Clone)]
pub struct ClassificationAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub subject: String,
    pub record_id: String,
    pub level: SensitivityLevel,
    pub kind: AccessKind,
    pub allowed: bool,
    pub reason: String,
}

impl ClassificationAuditEntry {
    pub fn from(
        tick: u64,
        tenant_id: impl Into<String>,
        subject: impl Into<String>,
        record_id: impl Into<String>,
        level: SensitivityLevel,
        kind: AccessKind,
        decision: &EnforcementDecision,
    ) -> Self {
        let (allowed, reason) = match decision {
            EnforcementDecision::Allow => (true, "allow".to_string()),
            EnforcementDecision::Deny(r) => (false, r.clone()),
        };
        Self {
            tick,
            tenant_id: tenant_id.into(),
            subject: subject.into(),
            record_id: record_id.into(),
            level,
            kind,
            allowed,
            reason,
        }
    }
}

#[derive(Debug, Default)]
pub struct ClassificationAuditLog {
    entries: VecDeque<ClassificationAuditEntry>,
}

impl ClassificationAuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: ClassificationAuditEntry) {
        self.entries.push_back(entry);
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn denied_for_tenant(&self, tenant_id: &str) -> Vec<&ClassificationAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id && !e.allowed)
            .collect()
    }

    pub fn allowed_for_tenant(&self, tenant_id: &str) -> Vec<&ClassificationAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id && e.allowed)
            .collect()
    }

    pub fn for_record(&self, record_id: &str) -> Vec<&ClassificationAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.record_id == record_id)
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &ClassificationAuditEntry> {
        self.entries.iter()
    }
}
