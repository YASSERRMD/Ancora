use crate::guardrail::{GuardrailOutcome, InputGuardrail, OutputGuardrail, ActionGuardrail};
use crate::journal::GuardrailJournal;

/// A composable guardrail policy attached to an agent.
pub struct GuardrailPolicy {
    pub input_guards: Vec<Box<dyn InputGuardrail>>,
    pub output_guards: Vec<Box<dyn OutputGuardrail>>,
    pub action_guards: Vec<Box<dyn ActionGuardrail>>,
}

impl GuardrailPolicy {
    pub fn new() -> Self {
        Self { input_guards: Vec::new(), output_guards: Vec::new(), action_guards: Vec::new() }
    }

    pub fn add_input<G: InputGuardrail + 'static>(&mut self, g: G) {
        self.input_guards.push(Box::new(g));
    }

    pub fn add_output<G: OutputGuardrail + 'static>(&mut self, g: G) {
        self.output_guards.push(Box::new(g));
    }

    pub fn add_action<G: ActionGuardrail + 'static>(&mut self, g: G) {
        self.action_guards.push(Box::new(g));
    }

    pub fn check_input(&self, input: &str, journal: &mut GuardrailJournal, tick: u64) -> GuardrailOutcome {
        for g in &self.input_guards {
            let outcome = g.check_input(input);
            if outcome != GuardrailOutcome::Pass {
                journal.record(tick, "input", input, outcome.clone());
                return outcome;
            }
        }
        GuardrailOutcome::Pass
    }

    pub fn check_output(&self, output: &str, journal: &mut GuardrailJournal, tick: u64) -> GuardrailOutcome {
        for g in &self.output_guards {
            let outcome = g.check_output(output);
            if outcome != GuardrailOutcome::Pass {
                journal.record(tick, "output", output, outcome.clone());
                return outcome;
            }
        }
        GuardrailOutcome::Pass
    }

    pub fn check_action(&self, tool: &str, input: &str, journal: &mut GuardrailJournal, tick: u64) -> GuardrailOutcome {
        for g in &self.action_guards {
            let outcome = g.check_action(tool, input);
            if outcome != GuardrailOutcome::Pass {
                journal.record(tick, "action", tool, outcome.clone());
                return outcome;
            }
        }
        GuardrailOutcome::Pass
    }
}

impl Default for GuardrailPolicy {
    fn default() -> Self {
        Self::new()
    }
}
