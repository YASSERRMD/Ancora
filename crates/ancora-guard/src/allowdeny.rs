use crate::guardrail::{ActionGuardrail, GuardrailOutcome};

/// Enforces an allowlist or denylist of tool names for action guardrailing.
pub struct AllowDenyGuardrail {
    pub allowlist: Option<Vec<String>>,
    pub denylist: Vec<String>,
}

impl AllowDenyGuardrail {
    pub fn allow_only(tools: Vec<&str>) -> Self {
        Self {
            allowlist: Some(tools.into_iter().map(|s| s.to_string()).collect()),
            denylist: Vec::new(),
        }
    }

    pub fn deny(tools: Vec<&str>) -> Self {
        Self {
            allowlist: None,
            denylist: tools.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl ActionGuardrail for AllowDenyGuardrail {
    fn check_action(&self, tool_name: &str, _input: &str) -> GuardrailOutcome {
        if self.denylist.iter().any(|d| d == tool_name) {
            return GuardrailOutcome::Block(format!("tool '{tool_name}' is on the denylist"));
        }
        if let Some(allow) = &self.allowlist {
            if !allow.iter().any(|a| a == tool_name) {
                return GuardrailOutcome::Block(format!("tool '{tool_name}' not on the allowlist"));
            }
        }
        GuardrailOutcome::Pass
    }
}
