use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackVector {
    Network,
    Local,
    Physical,
    Adjacent,
}

impl fmt::Display for AttackVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AttackVector::Network => "NETWORK",
            AttackVector::Local => "LOCAL",
            AttackVector::Physical => "PHYSICAL",
            AttackVector::Adjacent => "ADJACENT",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackOutcome {
    Success,
    PartialSuccess,
    Failure,
    Detected,
    Blocked,
}

impl fmt::Display for AttackOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AttackOutcome::Success => "SUCCESS",
            AttackOutcome::PartialSuccess => "PARTIAL_SUCCESS",
            AttackOutcome::Failure => "FAILURE",
            AttackOutcome::Detected => "DETECTED",
            AttackOutcome::Blocked => "BLOCKED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct AttackStep {
    pub id: String,
    pub scenario_id: String,
    pub name: String,
    pub vector: AttackVector,
    pub outcome: AttackOutcome,
    pub technique: String,
    pub detail: String,
    pub tick: u64,
}

impl AttackStep {
    pub fn new(
        id: impl Into<String>,
        scenario_id: impl Into<String>,
        name: impl Into<String>,
        vector: AttackVector,
        outcome: AttackOutcome,
        technique: impl Into<String>,
        detail: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            scenario_id: scenario_id.into(),
            name: name.into(),
            vector,
            outcome,
            technique: technique.into(),
            detail: detail.into(),
            tick,
        }
    }

    pub fn is_successful(&self) -> bool {
        matches!(self.outcome, AttackOutcome::Success | AttackOutcome::PartialSuccess)
    }

    pub fn was_detected(&self) -> bool { self.outcome == AttackOutcome::Detected }
    pub fn was_blocked(&self) -> bool { self.outcome == AttackOutcome::Blocked }
}

pub struct AttackLog {
    steps: Vec<AttackStep>,
}

impl AttackLog {
    pub fn new() -> Self { Self { steps: Vec::new() } }
    pub fn record(&mut self, step: AttackStep) { self.steps.push(step); }
    pub fn count(&self) -> usize { self.steps.len() }
    pub fn for_scenario<'a>(&'a self, scenario_id: &str) -> Vec<&'a AttackStep> {
        self.steps.iter().filter(|s| s.scenario_id == scenario_id).collect()
    }
    pub fn successful(&self) -> Vec<&AttackStep> {
        self.steps.iter().filter(|s| s.is_successful()).collect()
    }
    pub fn detected(&self) -> Vec<&AttackStep> {
        self.steps.iter().filter(|s| s.was_detected()).collect()
    }
    pub fn by_vector<'a>(&'a self, vector: &AttackVector) -> Vec<&'a AttackStep> {
        self.steps.iter().filter(|s| &s.vector == vector).collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &AttackStep> { self.steps.iter() }
}
