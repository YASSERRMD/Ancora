use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Success,
    Failure,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: u64,
    pub tick: u64,
    pub tenant_id: String,
    pub subject: String,
    pub operation: String,
    pub resource: String,
    pub outcome: Outcome,
    pub severity: Severity,
    pub details: HashMap<String, String>,
    pub checksum: u64,
}

impl AuditEntry {
    pub fn new(
        id: u64,
        tick: u64,
        tenant_id: impl Into<String>,
        subject: impl Into<String>,
        operation: impl Into<String>,
        resource: impl Into<String>,
        outcome: Outcome,
        severity: Severity,
    ) -> Self {
        let mut entry = Self {
            id,
            tick,
            tenant_id: tenant_id.into(),
            subject: subject.into(),
            operation: operation.into(),
            resource: resource.into(),
            outcome,
            severity,
            details: HashMap::new(),
            checksum: 0,
        };
        entry.checksum = entry.compute_checksum();
        entry
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.insert(key.into(), value.into());
        self.checksum = self.compute_checksum();
        self
    }

    pub fn compute_checksum(&self) -> u64 {
        let mut h: u64 = self.id.wrapping_mul(2654435761);
        h = h.wrapping_add(self.tick.wrapping_mul(2246822519));
        for b in self.tenant_id.bytes() { h = h.wrapping_add(b as u64).wrapping_mul(6364136223846793005); }
        for b in self.operation.bytes() { h = h.wrapping_add(b as u64).wrapping_mul(6364136223846793005); }
        h
    }

    pub fn verify(&self) -> bool {
        self.checksum == self.compute_checksum()
    }
}
