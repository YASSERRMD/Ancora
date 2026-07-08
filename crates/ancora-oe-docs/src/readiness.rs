//! Observability and evaluation readiness checklist.

/// The status of a single checklist item.
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Pass,
    Fail,
    NotApplicable,
}

impl std::fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckStatus::Pass => write!(f, "PASS"),
            CheckStatus::Fail => write!(f, "FAIL"),
            CheckStatus::NotApplicable => write!(f, "N/A"),
        }
    }
}

/// A single item in the readiness checklist.
#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub id: String,
    pub description: String,
    pub status: CheckStatus,
    pub notes: Option<String>,
}

impl ChecklistItem {
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            status: CheckStatus::Fail,
            notes: None,
        }
    }

    pub fn pass(mut self) -> Self {
        self.status = CheckStatus::Pass;
        self
    }

    pub fn na(mut self) -> Self {
        self.status = CheckStatus::NotApplicable;
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// The full readiness checklist for observability and evaluation.
#[derive(Debug, Default)]
pub struct ReadinessChecklist {
    pub items: Vec<ChecklistItem>,
}

impl ReadinessChecklist {
    pub fn add(&mut self, item: ChecklistItem) {
        self.items.push(item);
    }

    pub fn pass_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.status == CheckStatus::Pass)
            .count()
    }

    pub fn fail_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.status == CheckStatus::Fail)
            .count()
    }

    pub fn is_ready(&self) -> bool {
        self.fail_count() == 0
    }

    pub fn summary(&self) -> String {
        format!(
            "{}/{} checks passing (0 failures required for readiness)",
            self.pass_count(),
            self.items.len()
        )
    }
}

/// Builds the standard Ancora OE readiness checklist with all items in Fail state.
pub fn build_default_checklist() -> ReadinessChecklist {
    let mut checklist = ReadinessChecklist::default();
    let items = [
        (
            "obs-001",
            "Distributed tracing is enabled and exporting spans",
        ),
        ("obs-002", "Metrics collection is configured"),
        ("obs-003", "Log aggregation is connected"),
        ("obs-004", "Cost analytics are recording token usage"),
        ("obs-005", "Drift monitor is calibrated with a baseline"),
        (
            "obs-006",
            "Safety monitor is active with critical alert routing",
        ),
        ("obs-007", "PII redaction policies are applied to telemetry"),
        (
            "eval-001",
            "Eval platform is connected to the agent under test",
        ),
        (
            "eval-002",
            "At least one dataset with ground truth is loaded",
        ),
        (
            "eval-003",
            "Regression gates are defined with threshold values",
        ),
        ("eval-004", "A/B experiment tracking is configured"),
        ("eval-005", "Human feedback queue is draining to a store"),
        ("eval-006", "Continuous eval schedule is set for OnDeploy"),
        ("eval-007", "Dev studio is accessible for local replay"),
        (
            "eval-008",
            "Observability integrations are tested end-to-end",
        ),
    ];
    for (id, desc) in items {
        checklist.add(ChecklistItem::new(id, desc));
    }
    checklist
}
