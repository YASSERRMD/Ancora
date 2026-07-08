use crate::error::SkillError;
use crate::skill::SkillScope;
use serde_json::Value;

/// Descriptor for a sub-agent that can be invoked as a task node.
#[derive(Debug, Clone)]
pub struct SubAgentDescriptor {
    pub agent_id: String,
    pub skill_name: String,
    pub system_prompt: String,
    pub scope: SkillScope,
}

/// A node that invokes a sub-agent within the task graph.
#[derive(Debug, Clone)]
pub struct SubAgentNode {
    pub node_id: String,
    pub agent_id: String,
    pub input: Value,
}

/// Result of a sub-agent invocation.
#[derive(Debug, Clone)]
pub struct SubAgentResult {
    pub node_id: String,
    pub output: Value,
}

impl SubAgentNode {
    pub fn new(node_id: &str, agent_id: &str, input: Value) -> Self {
        Self {
            node_id: node_id.to_string(),
            agent_id: agent_id.to_string(),
            input,
        }
    }

    pub fn invoke(&self, scope: &SkillScope) -> Result<SubAgentResult, SkillError> {
        if *scope == SkillScope::Unrestricted {
            return Err(SkillError::PermissionDenied(
                "unrestricted scope not allowed for sub-agents".into(),
            ));
        }
        Ok(SubAgentResult {
            node_id: self.node_id.clone(),
            output: serde_json::json!({ "status": "ok", "agent": self.agent_id }),
        })
    }
}
