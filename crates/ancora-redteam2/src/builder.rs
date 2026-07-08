use crate::attack::{AttackOutcome, AttackStep, AttackVector};
use crate::objective::RedTeamObjective;
use crate::scenario::{RedTeamScenario, ScenarioKind};

pub struct ScenarioBuilder {
    id: String,
    tenant_id: String,
    name: String,
    kind: ScenarioKind,
    tick: u64,
    mitre: Option<String>,
}

impl ScenarioBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        name: impl Into<String>,
        kind: ScenarioKind,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            kind,
            tick,
            mitre: None,
        }
    }

    pub fn mitre(mut self, tactic: impl Into<String>) -> Self {
        self.mitre = Some(tactic.into());
        self
    }

    pub fn build(self) -> RedTeamScenario {
        let mut s = RedTeamScenario::new(self.id, self.tenant_id, self.name, self.kind, self.tick);
        if let Some(m) = self.mitre {
            s = s.with_mitre(m);
        }
        s
    }
}

pub struct AttackStepBuilder {
    id: String,
    scenario_id: String,
    name: String,
    vector: AttackVector,
    outcome: AttackOutcome,
    technique: String,
    detail: String,
    tick: u64,
}

impl AttackStepBuilder {
    pub fn new(
        id: impl Into<String>,
        scenario_id: impl Into<String>,
        name: impl Into<String>,
        vector: AttackVector,
        outcome: AttackOutcome,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            scenario_id: scenario_id.into(),
            name: name.into(),
            vector,
            outcome,
            technique: String::new(),
            detail: String::new(),
            tick,
        }
    }

    pub fn technique(mut self, t: impl Into<String>) -> Self {
        self.technique = t.into();
        self
    }
    pub fn detail(mut self, d: impl Into<String>) -> Self {
        self.detail = d.into();
        self
    }

    pub fn build(self) -> AttackStep {
        AttackStep::new(
            self.id,
            self.scenario_id,
            self.name,
            self.vector,
            self.outcome,
            self.technique,
            self.detail,
            self.tick,
        )
    }
}

pub struct ObjectiveBuilder {
    id: String,
    scenario_id: String,
    description: String,
}

impl ObjectiveBuilder {
    pub fn new(
        id: impl Into<String>,
        scenario_id: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            scenario_id: scenario_id.into(),
            description: description.into(),
        }
    }

    pub fn build(self) -> RedTeamObjective {
        RedTeamObjective::new(self.id, self.scenario_id, self.description)
    }
}
