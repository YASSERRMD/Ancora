/// Status of a readiness checklist item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckState {
    Done,
    InProgress,
    Blocked { reason: String },
    NotStarted,
}

impl std::fmt::Display for CheckState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckState::Done => write!(f, "[x]"),
            CheckState::InProgress => write!(f, "[~]"),
            CheckState::Blocked { reason } => write!(f, "[!] blocked: {}", reason),
            CheckState::NotStarted => write!(f, "[ ]"),
        }
    }
}

/// A single item in the release readiness checklist.
#[derive(Debug, Clone)]
pub struct ReadinessItem {
    pub label: String,
    pub state: CheckState,
    pub owner: Option<String>,
}

impl ReadinessItem {
    pub fn new(label: impl Into<String>, state: CheckState) -> Self {
        Self {
            label: label.into(),
            state,
            owner: None,
        }
    }

    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    pub fn is_done(&self) -> bool {
        self.state == CheckState::Done
    }

    pub fn render(&self) -> String {
        let owner_part = self
            .owner
            .as_deref()
            .map(|o| format!(" @{}", o))
            .unwrap_or_default();
        format!("{} {}{}", self.state, self.label, owner_part)
    }
}

/// Full readiness checklist.
#[derive(Debug, Default)]
pub struct ReadinessChecklist {
    pub items: Vec<ReadinessItem>,
}

impl ReadinessChecklist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, item: ReadinessItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn done_count(&self) -> usize {
        self.items.iter().filter(|i| i.is_done()).count()
    }

    pub fn total(&self) -> usize {
        self.items.len()
    }

    pub fn percent_done(&self) -> f64 {
        if self.items.is_empty() {
            return 100.0;
        }
        (self.done_count() as f64 / self.total() as f64) * 100.0
    }

    pub fn all_done(&self) -> bool {
        self.items.iter().all(|i| i.is_done())
    }

    pub fn render(&self) -> String {
        let mut out = format!(
            "Readiness: {}/{} done ({:.0}%)\n",
            self.done_count(),
            self.total(),
            self.percent_done()
        );
        for item in &self.items {
            out.push_str(&format!("  {}\n", item.render()));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_done_when_all_checked() {
        let checklist = ReadinessChecklist::new()
            .add(ReadinessItem::new("tests green", CheckState::Done))
            .add(ReadinessItem::new("docs complete", CheckState::Done));
        assert!(checklist.all_done());
        assert_eq!(checklist.percent_done() as u32, 100);
    }

    #[test]
    fn partial_done() {
        let checklist = ReadinessChecklist::new()
            .add(ReadinessItem::new("tests", CheckState::Done))
            .add(ReadinessItem::new("perf", CheckState::InProgress));
        assert!(!checklist.all_done());
        assert_eq!(checklist.done_count(), 1);
    }

    #[test]
    fn blocked_item_renders_reason() {
        let item = ReadinessItem::new(
            "deploy",
            CheckState::Blocked {
                reason: "waiting for infra".into(),
            },
        );
        let r = item.render();
        assert!(r.contains("waiting for infra"));
    }
}
