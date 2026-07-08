use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectiveStatus {
    Pending,
    InProgress,
    Achieved,
    Failed,
}

impl fmt::Display for ObjectiveStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ObjectiveStatus::Pending => "PENDING",
            ObjectiveStatus::InProgress => "IN_PROGRESS",
            ObjectiveStatus::Achieved => "ACHIEVED",
            ObjectiveStatus::Failed => "FAILED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct RedTeamObjective {
    pub id: String,
    pub scenario_id: String,
    pub description: String,
    pub status: ObjectiveStatus,
    pub achieved_tick: Option<u64>,
}

impl RedTeamObjective {
    pub fn new(
        id: impl Into<String>,
        scenario_id: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            scenario_id: scenario_id.into(),
            description: description.into(),
            status: ObjectiveStatus::Pending,
            achieved_tick: None,
        }
    }

    pub fn start(&mut self) {
        self.status = ObjectiveStatus::InProgress;
    }
    pub fn achieve(&mut self, tick: u64) {
        self.status = ObjectiveStatus::Achieved;
        self.achieved_tick = Some(tick);
    }
    pub fn fail(&mut self) {
        self.status = ObjectiveStatus::Failed;
    }
    pub fn is_achieved(&self) -> bool {
        self.status == ObjectiveStatus::Achieved
    }
    pub fn is_done(&self) -> bool {
        self.status != ObjectiveStatus::Pending && self.status != ObjectiveStatus::InProgress
    }
}

pub struct ObjectiveTracker {
    objectives: Vec<RedTeamObjective>,
}

impl Default for ObjectiveTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectiveTracker {
    pub fn new() -> Self {
        Self {
            objectives: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: RedTeamObjective) {
        self.objectives.push(obj);
    }
    pub fn count(&self) -> usize {
        self.objectives.len()
    }
    pub fn achieved_count(&self) -> usize {
        self.objectives.iter().filter(|o| o.is_achieved()).count()
    }
    pub fn pending_count(&self) -> usize {
        self.objectives
            .iter()
            .filter(|o| o.status == ObjectiveStatus::Pending)
            .count()
    }
    pub fn for_scenario<'a>(&'a self, scenario_id: &str) -> Vec<&'a RedTeamObjective> {
        self.objectives
            .iter()
            .filter(|o| o.scenario_id == scenario_id)
            .collect()
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut RedTeamObjective> {
        self.objectives.iter_mut().find(|o| o.id == id)
    }
    pub fn progress(&self) -> f64 {
        if self.objectives.is_empty() {
            return 0.0;
        }
        self.achieved_count() as f64 / self.objectives.len() as f64
    }
}
