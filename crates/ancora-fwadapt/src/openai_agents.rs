//! Bridge between the OpenAI Agents SDK handoff protocol and Ancora.
//!
//! The OpenAI Agents SDK routes work between agents via "handoffs" - a typed
//! message that names the target agent and carries a context payload. This
//! module models that handoff protocol and maps it onto Ancora's dispatcher.

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAIHandoff {
    pub target_agent: String,
    pub reason: String,
    pub context: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAIAgentResult {
    pub agent_id: String,
    pub output: String,
    pub finished: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HandoffError {
    NoTargetRegistered(String),
    ExecutionFailed(String),
}

impl std::fmt::Display for HandoffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTargetRegistered(a) => write!(f, "no agent registered for: {}", a),
            Self::ExecutionFailed(r) => write!(f, "execution failed: {}", r),
        }
    }
}

pub type AgentFn = fn(&str) -> Result<OpenAIAgentResult, HandoffError>;

/// A bridge that maps OpenAI SDK handoffs onto registered Ancora agent functions.
pub struct HandoffBridge {
    agents: Vec<(String, AgentFn)>,
}

impl HandoffBridge {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Register an Ancora agent as a handoff target.
    pub fn register_agent(&mut self, name: impl Into<String>, func: AgentFn) {
        self.agents.push((name.into(), func));
    }

    /// Execute a handoff by delegating to the registered agent.
    pub fn execute_handoff(
        &self,
        handoff: &OpenAIHandoff,
    ) -> Result<OpenAIAgentResult, HandoffError> {
        for (name, func) in &self.agents {
            if *name == handoff.target_agent {
                return func(&handoff.context);
            }
        }
        Err(HandoffError::NoTargetRegistered(
            handoff.target_agent.clone(),
        ))
    }

    /// List all registered agent names.
    pub fn agent_names(&self) -> Vec<&str> {
        self.agents.iter().map(|(n, _)| n.as_str()).collect()
    }
}

impl Default for HandoffBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a handoff descriptor.
pub fn build_handoff(target: &str, reason: &str, context: &str) -> OpenAIHandoff {
    OpenAIHandoff {
        target_agent: target.to_string(),
        reason: reason.to_string(),
        context: context.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summariser(ctx: &str) -> Result<OpenAIAgentResult, HandoffError> {
        Ok(OpenAIAgentResult {
            agent_id: "summariser".into(),
            output: format!("summary of: {}", ctx),
            finished: true,
        })
    }

    #[test]
    fn handoff_executes_correctly() {
        let mut bridge = HandoffBridge::new();
        bridge.register_agent("summariser", summariser);
        let h = build_handoff("summariser", "needs summarisation", "long text");
        let res = bridge.execute_handoff(&h).unwrap();
        assert!(res.finished);
        assert!(res.output.contains("long text"));
    }
}
