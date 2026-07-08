use crate::connection::ConnectionRequest;
use crate::evaluator::PolicyDecision;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct EvaluationRecord {
    pub tick: u64,
    pub tenant_id: String,
    pub source: String,
    pub host: String,
    pub port: u16,
    pub allowed: bool,
    pub reason: String,
}

impl EvaluationRecord {
    pub fn from(tick: u64, req: &ConnectionRequest, decision: &PolicyDecision) -> Self {
        let (allowed, reason) = match decision {
            PolicyDecision::Allow => (true, "allow".to_string()),
            PolicyDecision::Deny(r) => (false, r.clone()),
            PolicyDecision::NoPolicy => (false, "no policy".to_string()),
        };
        Self {
            tick,
            tenant_id: req.tenant_id.clone(),
            source: req.source.clone(),
            host: req.destination_host.clone(),
            port: req.destination_port,
            allowed,
            reason,
        }
    }
}

#[derive(Debug, Default)]
pub struct NetpolAuditLog {
    records: VecDeque<EvaluationRecord>,
}

impl NetpolAuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: EvaluationRecord) {
        self.records.push_back(entry);
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn denied_for(&self, tenant_id: &str) -> Vec<&EvaluationRecord> {
        self.records
            .iter()
            .filter(|r| r.tenant_id == tenant_id && !r.allowed)
            .collect()
    }

    pub fn allowed_for(&self, tenant_id: &str) -> Vec<&EvaluationRecord> {
        self.records
            .iter()
            .filter(|r| r.tenant_id == tenant_id && r.allowed)
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &EvaluationRecord> {
        self.records.iter()
    }
}
