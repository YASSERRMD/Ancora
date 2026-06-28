use crate::chain::BootChain;
use crate::policy::BootPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainIssue {
    EmptyChain,
    MissingRequiredKind(String),
    DuplicateId(String),
}

impl std::fmt::Display for ChainIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainIssue::EmptyChain => write!(f, "boot chain has no measurements"),
            ChainIssue::MissingRequiredKind(k) => write!(f, "missing required kind: {}", k),
            ChainIssue::DuplicateId(id) => write!(f, "duplicate measurement id: {}", id),
        }
    }
}

pub struct ChainValidator;

impl ChainValidator {
    pub fn validate(chain: &BootChain, policy: &BootPolicy) -> Vec<ChainIssue> {
        let mut issues = Vec::new();
        if chain.is_empty() {
            issues.push(ChainIssue::EmptyChain);
            return issues;
        }
        let present = chain.present_kinds();
        for kind in &policy.require_kinds {
            if !present.contains(kind.as_str()) {
                issues.push(ChainIssue::MissingRequiredKind(kind.clone()));
            }
        }
        let mut seen_ids = std::collections::HashSet::new();
        for step in chain.steps() {
            if !seen_ids.insert(step.id.clone()) {
                issues.push(ChainIssue::DuplicateId(step.id.clone()));
            }
        }
        issues
    }

    pub fn is_valid(chain: &BootChain, policy: &BootPolicy) -> bool {
        Self::validate(chain, policy).is_empty()
    }
}
