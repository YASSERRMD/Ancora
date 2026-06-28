//! Reasoning journal: deterministic trace of all reasoning events.

#[derive(Debug, Clone, PartialEq)]
pub enum ReasoningEvent {
    StepAdded { index: usize, claim: String },
    StepVerified { index: usize },
    StepRefuted { index: usize },
    StepAbstained { index: usize },
    ContradictionFound { a: usize, b: usize },
    FactChecked { claim: String, grounded: bool },
    CitationAdded { claim: String, citation: String },
}

#[derive(Default)]
pub struct ReasoningJournal {
    events: Vec<(u64, ReasoningEvent)>,
}

impl ReasoningJournal {
    pub fn record(&mut self, tick: u64, event: ReasoningEvent) {
        self.events.push((tick, event));
    }

    pub fn events(&self) -> &[(u64, ReasoningEvent)] {
        &self.events
    }

    pub fn replay(&self) -> Vec<&ReasoningEvent> {
        self.events.iter().map(|(_, e)| e).collect()
    }
}
