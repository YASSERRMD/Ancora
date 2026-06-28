/// Status of the full observability and eval test suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuiteResult {
    Green,
    Failed { failed_count: usize, total: usize },
}

#[derive(Debug, Clone)]
pub struct SuiteStatus {
    pub name: String,
    pub result: SuiteResult,
    pub notes: Option<String>,
}

impl SuiteStatus {
    pub fn new(name: impl Into<String>, result: SuiteResult) -> Self {
        Self {
            name: name.into(),
            result,
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn is_green(&self) -> bool {
        self.result == SuiteResult::Green
    }

    pub fn summary(&self) -> String {
        match &self.result {
            SuiteResult::Green => format!("{}: GREEN", self.name),
            SuiteResult::Failed { failed_count, total } => {
                format!("{}: FAILED ({}/{})", self.name, failed_count, total)
            }
        }
    }
}

/// Collect all suite statuses and report whether the milestone is green.
pub fn milestone_green(statuses: &[SuiteStatus]) -> bool {
    statuses.iter().all(|s| s.is_green())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn green_status_is_green() {
        let s = SuiteStatus::new("obs-suite", SuiteResult::Green);
        assert!(s.is_green());
        assert!(s.summary().contains("GREEN"));
    }

    #[test]
    fn failed_status_not_green() {
        let s = SuiteStatus::new("eval-suite", SuiteResult::Failed { failed_count: 2, total: 20 });
        assert!(!s.is_green());
        assert!(s.summary().contains("FAILED"));
    }

    #[test]
    fn milestone_green_all_pass() {
        let statuses = vec![
            SuiteStatus::new("a", SuiteResult::Green),
            SuiteStatus::new("b", SuiteResult::Green),
        ];
        assert!(milestone_green(&statuses));
    }

    #[test]
    fn milestone_not_green_if_any_fail() {
        let statuses = vec![
            SuiteStatus::new("a", SuiteResult::Green),
            SuiteStatus::new("b", SuiteResult::Failed { failed_count: 1, total: 5 }),
        ];
        assert!(!milestone_green(&statuses));
    }
}
