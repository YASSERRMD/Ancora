use crate::measurement::Measurement;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainStatus {
    Valid,
    Broken,
    Incomplete,
}

pub struct BootChain {
    pub tenant_id: String,
    pub node_id: String,
    steps: Vec<Measurement>,
}

impl BootChain {
    pub fn new(tenant_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            node_id: node_id.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, m: Measurement) {
        self.steps.push(m);
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    pub fn steps(&self) -> &[Measurement] {
        &self.steps
    }

    pub fn status(&self) -> ChainStatus {
        if self.steps.is_empty() {
            return ChainStatus::Incomplete;
        }
        ChainStatus::Valid
    }

    pub fn present_kinds(&self) -> HashSet<String> {
        self.steps.iter().map(|s| format!("{}", s.kind)).collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Measurement> {
        self.steps.iter().find(|s| s.id == id)
    }

    pub fn digest_at(&self, index: usize) -> Option<&str> {
        self.steps.get(index).map(|s| s.digest.as_str())
    }
}
