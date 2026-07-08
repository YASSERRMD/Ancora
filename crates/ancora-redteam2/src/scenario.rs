use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioKind {
    PrivilegeEscalation,
    LateralMovement,
    DataExfiltration,
    CredentialHarvesting,
    PersistenceMechanism,
    DefenseEvasion,
    CommandAndControl,
    InitialAccess,
    CollectionAndRecon,
    ImpactAndDisruption,
}

impl fmt::Display for ScenarioKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ScenarioKind::PrivilegeEscalation => "PRIVILEGE_ESCALATION",
            ScenarioKind::LateralMovement => "LATERAL_MOVEMENT",
            ScenarioKind::DataExfiltration => "DATA_EXFILTRATION",
            ScenarioKind::CredentialHarvesting => "CREDENTIAL_HARVESTING",
            ScenarioKind::PersistenceMechanism => "PERSISTENCE_MECHANISM",
            ScenarioKind::DefenseEvasion => "DEFENSE_EVASION",
            ScenarioKind::CommandAndControl => "COMMAND_AND_CONTROL",
            ScenarioKind::InitialAccess => "INITIAL_ACCESS",
            ScenarioKind::CollectionAndRecon => "COLLECTION_AND_RECON",
            ScenarioKind::ImpactAndDisruption => "IMPACT_AND_DISRUPTION",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Aborted,
}

impl fmt::Display for ScenarioStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ScenarioStatus::Pending => "PENDING",
            ScenarioStatus::Running => "RUNNING",
            ScenarioStatus::Completed => "COMPLETED",
            ScenarioStatus::Failed => "FAILED",
            ScenarioStatus::Aborted => "ABORTED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct RedTeamScenario {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub kind: ScenarioKind,
    pub status: ScenarioStatus,
    pub mitre_tactic: Option<String>,
    pub created_tick: u64,
    pub completed_tick: Option<u64>,
    pub metadata: HashMap<String, String>,
}

impl RedTeamScenario {
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
            status: ScenarioStatus::Pending,
            mitre_tactic: None,
            created_tick: tick,
            completed_tick: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_mitre(mut self, tactic: impl Into<String>) -> Self {
        self.mitre_tactic = Some(tactic.into());
        self
    }

    pub fn with_metadata(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.metadata.insert(k.into(), v.into());
        self
    }

    pub fn start(&mut self) {
        self.status = ScenarioStatus::Running;
    }
    pub fn complete(&mut self, tick: u64) {
        self.status = ScenarioStatus::Completed;
        self.completed_tick = Some(tick);
    }
    pub fn fail(&mut self) {
        self.status = ScenarioStatus::Failed;
    }
    pub fn abort(&mut self) {
        self.status = ScenarioStatus::Aborted;
    }

    pub fn is_active(&self) -> bool {
        self.status == ScenarioStatus::Running
    }
    pub fn is_done(&self) -> bool {
        matches!(
            self.status,
            ScenarioStatus::Completed | ScenarioStatus::Failed | ScenarioStatus::Aborted
        )
    }

    pub fn duration(&self, current_tick: u64) -> u64 {
        match self.completed_tick {
            Some(t) => t.saturating_sub(self.created_tick),
            None => current_tick.saturating_sub(self.created_tick),
        }
    }
}
