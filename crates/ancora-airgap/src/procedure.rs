use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcedureStepStatus {
    Pending,
    Completed,
    Skipped,
    Failed,
}

impl fmt::Display for ProcedureStepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProcedureStepStatus::Pending => "PENDING",
            ProcedureStepStatus::Completed => "COMPLETED",
            ProcedureStepStatus::Skipped => "SKIPPED",
            ProcedureStepStatus::Failed => "FAILED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ProcedureStep {
    pub id: String,
    pub title: String,
    pub instructions: String,
    pub status: ProcedureStepStatus,
    pub completed_tick: Option<u64>,
}

impl ProcedureStep {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            instructions: instructions.into(),
            status: ProcedureStepStatus::Pending,
            completed_tick: None,
        }
    }

    pub fn complete(&mut self, tick: u64) {
        self.status = ProcedureStepStatus::Completed;
        self.completed_tick = Some(tick);
    }

    pub fn skip(&mut self) {
        self.status = ProcedureStepStatus::Skipped;
    }
    pub fn fail(&mut self) {
        self.status = ProcedureStepStatus::Failed;
    }
    pub fn is_done(&self) -> bool {
        self.status != ProcedureStepStatus::Pending
    }
}

pub struct OfflineProcedure {
    pub id: String,
    pub name: String,
    pub tenant_id: String,
    steps: Vec<ProcedureStep>,
}

impl OfflineProcedure {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        tenant_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            tenant_id: tenant_id.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: ProcedureStep) {
        self.steps.push(step);
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn completed_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status == ProcedureStepStatus::Completed)
            .count()
    }

    pub fn pending_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status == ProcedureStepStatus::Pending)
            .count()
    }

    pub fn is_complete(&self) -> bool {
        !self.steps.is_empty() && self.steps.iter().all(|s| s.is_done())
    }

    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.completed_count() as f64 / self.steps.len() as f64
    }

    pub fn get_step_mut(&mut self, id: &str) -> Option<&mut ProcedureStep> {
        self.steps.iter_mut().find(|s| s.id == id)
    }

    pub fn steps(&self) -> &[ProcedureStep] {
        &self.steps
    }
}
