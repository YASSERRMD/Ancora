//! Evidence tracking: store and retrieve supporting sources per claim.

use std::collections::HashMap;

#[derive(Default)]
pub struct EvidenceStore {
    evidence: HashMap<String, Vec<String>>,
}

impl EvidenceStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, claim: &str, source: String) {
        self.evidence
            .entry(claim.to_string())
            .or_default()
            .push(source);
    }

    pub fn get(&self, claim: &str) -> &[String] {
        self.evidence
            .get(claim)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn count(&self, claim: &str) -> usize {
        self.get(claim).len()
    }
}
