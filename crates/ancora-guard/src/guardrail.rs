/// Outcome of a guardrail check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardrailOutcome {
    Pass,
    Block(String),
    Repair(String),
}

/// Input guardrail interface.
pub trait InputGuardrail {
    fn check_input(&self, input: &str) -> GuardrailOutcome;
}

/// Output guardrail interface.
pub trait OutputGuardrail {
    fn check_output(&self, output: &str) -> GuardrailOutcome;
}

/// Action guardrail before tool execution.
pub trait ActionGuardrail {
    fn check_action(&self, tool_name: &str, input: &str) -> GuardrailOutcome;
}
