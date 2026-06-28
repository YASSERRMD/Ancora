use crate::engine::Decision;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AbacDecisionRecord {
    pub action: String,
    pub decision: String,
    pub policy_id: Option<String>,
    pub tick: u64,
}

impl AbacDecisionRecord {
    pub fn from_decision(action: &str, decision: &Decision, tick: u64) -> Self {
        let (decision_str, policy_id) = match decision {
            Decision::Allow => ("allow".into(), None),
            Decision::Deny(reason) => ("deny".into(), Some(reason.clone())),
            Decision::NotApplicable => ("not_applicable".into(), None),
        };
        Self { action: action.to_string(), decision: decision_str, policy_id, tick }
    }
}

#[derive(Debug, Default)]
pub struct AbacAuditLog {
    records: VecDeque<AbacDecisionRecord>,
    max_size: usize,
}

impl AbacAuditLog {
    pub fn new(max_size: usize) -> Self { Self { records: VecDeque::new(), max_size } }

    pub fn record(&mut self, entry: AbacDecisionRecord) {
        if self.records.len() >= self.max_size { self.records.pop_front(); }
        self.records.push_back(entry);
    }

    pub fn count(&self) -> usize { self.records.len() }

    pub fn deny_count(&self) -> usize {
        self.records.iter().filter(|r| r.decision == "deny").count()
    }
}
