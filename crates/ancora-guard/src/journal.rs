use crate::guardrail::GuardrailOutcome;

/// A single recorded guardrail decision.
#[derive(Debug, Clone)]
pub struct GuardrailDecision {
    pub tick: u64,
    pub guardrail: String,
    pub input_snippet: String,
    pub outcome: GuardrailOutcome,
}

/// Append-only journal of guardrail decisions for replay.
#[derive(Debug, Default)]
pub struct GuardrailJournal {
    decisions: Vec<GuardrailDecision>,
}

impl GuardrailJournal {
    pub fn record(&mut self, tick: u64, guardrail: &str, snippet: &str, outcome: GuardrailOutcome) {
        self.decisions.push(GuardrailDecision {
            tick,
            guardrail: guardrail.to_string(),
            input_snippet: snippet.to_string(),
            outcome,
        });
    }

    pub fn decisions(&self) -> &[GuardrailDecision] {
        &self.decisions
    }

    pub fn blocked_count(&self) -> usize {
        self.decisions.iter().filter(|d| matches!(d.outcome, GuardrailOutcome::Block(_))).count()
    }

    pub fn repaired_count(&self) -> usize {
        self.decisions.iter().filter(|d| matches!(d.outcome, GuardrailOutcome::Repair(_))).count()
    }
}
