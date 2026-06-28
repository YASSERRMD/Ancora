use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Skipped,
    Failed,
}

impl fmt::Display for StepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StepStatus::Pending => "PENDING",
            StepStatus::InProgress => "IN_PROGRESS",
            StepStatus::Completed => "COMPLETED",
            StepStatus::Skipped => "SKIPPED",
            StepStatus::Failed => "FAILED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct RunbookStep {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: StepStatus,
    pub assignee: Option<String>,
    pub completed_tick: Option<u64>,
}

impl RunbookStep {
    pub fn new(id: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            status: StepStatus::Pending,
            assignee: None,
            completed_tick: None,
        }
    }

    pub fn complete(&mut self, tick: u64) {
        self.status = StepStatus::Completed;
        self.completed_tick = Some(tick);
    }

    pub fn skip(&mut self) { self.status = StepStatus::Skipped; }
    pub fn fail(&mut self) { self.status = StepStatus::Failed; }
    pub fn start(&mut self) { self.status = StepStatus::InProgress; }

    pub fn is_done(&self) -> bool {
        matches!(self.status, StepStatus::Completed | StepStatus::Skipped)
    }
}

#[derive(Debug, Clone)]
pub struct Runbook {
    pub id: String,
    pub name: String,
    pub incident_id: String,
    pub steps: Vec<RunbookStep>,
    pub metadata: HashMap<String, String>,
}

impl Runbook {
    pub fn new(id: impl Into<String>, name: impl Into<String>, incident_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            incident_id: incident_id.into(),
            steps: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_step(&mut self, step: RunbookStep) { self.steps.push(step); }

    pub fn step_count(&self) -> usize { self.steps.len() }

    pub fn completed_count(&self) -> usize {
        self.steps.iter().filter(|s| s.is_done()).count()
    }

    pub fn pending_count(&self) -> usize {
        self.steps.iter().filter(|s| s.status == StepStatus::Pending).count()
    }

    pub fn is_complete(&self) -> bool {
        !self.steps.is_empty() && self.steps.iter().all(|s| s.is_done())
    }

    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() { return 0.0; }
        self.completed_count() as f64 / self.step_count() as f64
    }

    pub fn get_step_mut(&mut self, id: &str) -> Option<&mut RunbookStep> {
        self.steps.iter_mut().find(|s| s.id == id)
    }
}
