use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedTeamAction {
    ScenarioCreated,
    ScenarioStarted,
    ScenarioCompleted,
    ScenarioAborted,
    AttackStepExecuted,
    ObjectiveAchieved,
    DetectionLogged,
    ReportGenerated,
}

impl fmt::Display for RedTeamAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RedTeamAction::ScenarioCreated => "SCENARIO_CREATED",
            RedTeamAction::ScenarioStarted => "SCENARIO_STARTED",
            RedTeamAction::ScenarioCompleted => "SCENARIO_COMPLETED",
            RedTeamAction::ScenarioAborted => "SCENARIO_ABORTED",
            RedTeamAction::AttackStepExecuted => "ATTACK_STEP_EXECUTED",
            RedTeamAction::ObjectiveAchieved => "OBJECTIVE_ACHIEVED",
            RedTeamAction::DetectionLogged => "DETECTION_LOGGED",
            RedTeamAction::ReportGenerated => "REPORT_GENERATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct RedTeamAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub scenario_id: String,
    pub action: RedTeamAction,
    pub actor: String,
    pub detail: String,
}

impl RedTeamAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        scenario_id: impl Into<String>,
        action: RedTeamAction,
        actor: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            scenario_id: scenario_id.into(),
            action,
            actor: actor.into(),
            detail: detail.into(),
        }
    }
}

pub struct RedTeamAuditLog {
    entries: VecDeque<RedTeamAuditEntry>,
}

impl RedTeamAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: RedTeamAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a RedTeamAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn for_scenario<'a>(&'a self, scenario_id: &str) -> Vec<&'a RedTeamAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.scenario_id == scenario_id)
            .collect()
    }
    pub fn by_action<'a>(&'a self, action: &RedTeamAction) -> Vec<&'a RedTeamAuditEntry> {
        self.entries
            .iter()
            .filter(|e| &e.action == action)
            .collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &RedTeamAuditEntry> {
        self.entries.iter()
    }
}
