use crate::error::CoordError;

/// A typed inter-agent contract specifying obligations.
#[derive(Debug, Clone)]
pub struct AgentContract {
    pub contract_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub obligations: Vec<String>,
}

impl AgentContract {
    pub fn new(
        contract_id: &str,
        from_agent: &str,
        to_agent: &str,
        obligations: Vec<&str>,
    ) -> Self {
        Self {
            contract_id: contract_id.to_string(),
            from_agent: from_agent.to_string(),
            to_agent: to_agent.to_string(),
            obligations: obligations.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn verify_fulfilled(&self, fulfilled: &[&str]) -> Result<(), CoordError> {
        for obligation in &self.obligations {
            if !fulfilled.contains(&obligation.as_str()) {
                return Err(CoordError::ContractViolation {
                    contract_id: self.contract_id.clone(),
                    obligation: obligation.clone(),
                });
            }
        }
        Ok(())
    }
}
