use crate::error::SynthError;

/// Approval gate for new synthesized tools.
#[derive(Debug, Default)]
pub struct ApprovalGate {
    approved: std::collections::HashSet<String>,
}

impl ApprovalGate {
    pub fn approve(&mut self, tool_name: &str) {
        self.approved.insert(tool_name.to_string());
    }

    pub fn revoke(&mut self, tool_name: &str) {
        self.approved.remove(tool_name);
    }

    pub fn check(&self, tool_name: &str) -> Result<(), SynthError> {
        if self.approved.contains(tool_name) {
            Ok(())
        } else {
            Err(SynthError::NotApproved(tool_name.to_string()))
        }
    }

    pub fn is_approved(&self, tool_name: &str) -> bool {
        self.approved.contains(tool_name)
    }
}
