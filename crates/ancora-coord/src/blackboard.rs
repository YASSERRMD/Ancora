use std::collections::HashMap;

/// Shared blackboard state visible to all agents in a coordination group.
#[derive(Debug, Default)]
pub struct Blackboard {
    entries: HashMap<String, String>,
    roles: HashMap<String, String>,
}

impl Blackboard {
    pub fn write(&mut self, agent_id: &str, key: &str, value: &str) -> Result<(), BlackboardError> {
        if let Some(owner) = self.roles.get(key) {
            if owner != agent_id {
                return Err(BlackboardError::PermissionDenied {
                    key: key.to_string(),
                    owner: owner.clone(),
                });
            }
        }
        self.entries.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn read(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }

    pub fn claim_role(&mut self, agent_id: &str, key: &str) {
        self.roles.insert(key.to_string(), agent_id.to_string());
    }

    pub fn entries_count(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlackboardError {
    PermissionDenied { key: String, owner: String },
}
