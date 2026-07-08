//! Test plan: structured acceptance checklist for ecosystem e2e.

#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Pass,
    Fail(String),
    Pending,
}

#[derive(Debug, Clone)]
pub struct AcceptanceCheck {
    pub id: String,
    pub description: String,
    pub status: CheckStatus,
}

impl AcceptanceCheck {
    pub fn new(id: &str, description: &str) -> Self {
        AcceptanceCheck {
            id: id.to_string(),
            description: description.to_string(),
            status: CheckStatus::Pending,
        }
    }

    pub fn pass(&mut self) {
        self.status = CheckStatus::Pass;
    }

    pub fn fail(&mut self, reason: &str) {
        self.status = CheckStatus::Fail(reason.to_string());
    }

    pub fn is_passing(&self) -> bool {
        matches!(self.status, CheckStatus::Pass)
    }
}

#[derive(Debug, Default)]
pub struct TestPlan {
    checks: Vec<AcceptanceCheck>,
}

impl TestPlan {
    pub fn new() -> Self {
        TestPlan { checks: Vec::new() }
    }

    pub fn add_check(&mut self, check: AcceptanceCheck) {
        self.checks.push(check);
    }

    pub fn pass_check(&mut self, id: &str) -> bool {
        if let Some(check) = self.checks.iter_mut().find(|c| c.id == id) {
            check.pass();
            return true;
        }
        false
    }

    pub fn fail_check(&mut self, id: &str, reason: &str) -> bool {
        if let Some(check) = self.checks.iter_mut().find(|c| c.id == id) {
            check.fail(reason);
            return true;
        }
        false
    }

    pub fn all_passing(&self) -> bool {
        !self.checks.is_empty() && self.checks.iter().all(|c| c.is_passing())
    }

    pub fn passing_count(&self) -> usize {
        self.checks.iter().filter(|c| c.is_passing()).count()
    }

    pub fn total_count(&self) -> usize {
        self.checks.len()
    }

    pub fn pending_checks(&self) -> Vec<&AcceptanceCheck> {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, CheckStatus::Pending))
            .collect()
    }

    pub fn failed_checks(&self) -> Vec<&AcceptanceCheck> {
        self.checks
            .iter()
            .filter(|c| matches!(c.status, CheckStatus::Fail(_)))
            .collect()
    }
}
