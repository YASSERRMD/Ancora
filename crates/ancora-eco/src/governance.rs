/// Governance roles in the extension ecosystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceRole {
    /// A core team member with full write access to the ecosystem.
    CoreMaintainer,
    /// A community contributor with limited write access.
    Contributor,
    /// A read-only observer with no write access.
    Observer,
}

/// A governance decision recorded in the ecosystem.
#[derive(Debug, Clone)]
pub struct GovernanceDecision {
    pub id: u64,
    pub title: String,
    pub decided_by: GovernanceRole,
    pub approved: bool,
}

impl GovernanceDecision {
    pub fn new(
        id: u64,
        title: impl Into<String>,
        decided_by: GovernanceRole,
        approved: bool,
    ) -> Self {
        GovernanceDecision {
            id,
            title: title.into(),
            decided_by,
            approved,
        }
    }
}

/// Governance council that manages decisions.
#[derive(Debug, Default)]
pub struct GovernanceCouncil {
    decisions: Vec<GovernanceDecision>,
}

impl GovernanceCouncil {
    pub fn new() -> Self {
        GovernanceCouncil { decisions: Vec::new() }
    }

    /// Record a decision.
    pub fn record(&mut self, decision: GovernanceDecision) {
        self.decisions.push(decision);
    }

    /// Return all approved decisions.
    pub fn approved_decisions(&self) -> Vec<&GovernanceDecision> {
        self.decisions.iter().filter(|d| d.approved).collect()
    }

    /// Return all decisions.
    pub fn all_decisions(&self) -> &[GovernanceDecision] {
        &self.decisions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_decision_counted() {
        let mut council = GovernanceCouncil::new();
        council.record(GovernanceDecision::new(
            1,
            "Adopt RFC-001",
            GovernanceRole::CoreMaintainer,
            true,
        ));
        council.record(GovernanceDecision::new(
            2,
            "Reject RFC-002",
            GovernanceRole::CoreMaintainer,
            false,
        ));
        assert_eq!(council.approved_decisions().len(), 1);
    }
}
