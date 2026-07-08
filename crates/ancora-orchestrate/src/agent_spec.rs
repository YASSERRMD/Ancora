use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    Orchestrator,
    Subagent,
    Critic,
    Synthesizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub agent_id: String,
    pub role: AgentRole,
    pub system_prompt: String,
    pub tools: Vec<String>,
    pub max_turns: u32,
    pub model: String,
}

impl AgentSpec {
    pub fn new(agent_id: &str, role: AgentRole, system_prompt: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            role,
            system_prompt: system_prompt.to_string(),
            tools: vec![],
            max_turns: 10,
            model: "claude-sonnet-4-6".to_string(),
        }
    }

    pub fn with_tools(mut self, tools: Vec<&str>) -> Self {
        self.tools = tools.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub task_id: String,
    pub agent_id: String,
    pub input: Value,
    pub parent_task_id: Option<String>,
}

impl AgentTask {
    pub fn new(task_id: &str, agent_id: &str, input: Value) -> Self {
        Self {
            task_id: task_id.to_string(),
            agent_id: agent_id.to_string(),
            input,
            parent_task_id: None,
        }
    }

    pub fn with_parent(mut self, parent: &str) -> Self {
        self.parent_task_id = Some(parent.to_string());
        self
    }
}
