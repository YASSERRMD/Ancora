use crate::guardrail::{InputGuardrail, GuardrailOutcome};

/// A custom guardrail registered with a user-provided closure.
pub struct CustomInputGuardrail {
    pub name: String,
    check_fn: Box<dyn Fn(&str) -> GuardrailOutcome + Send + Sync>,
}

impl CustomInputGuardrail {
    pub fn new(name: &str, f: impl Fn(&str) -> GuardrailOutcome + Send + Sync + 'static) -> Self {
        Self { name: name.to_string(), check_fn: Box::new(f) }
    }
}

impl InputGuardrail for CustomInputGuardrail {
    fn check_input(&self, input: &str) -> GuardrailOutcome {
        (self.check_fn)(input)
    }
}
