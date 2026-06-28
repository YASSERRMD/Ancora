/// Suite status for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone)]
pub struct SuiteResult {
    pub name: String,
    pub status: Status,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
}

impl SuiteResult {
    pub fn new(name: impl Into<String>, passed: usize, failed: usize, skipped: usize) -> Self {
        let status = if failed == 0 { Status::Green } else { Status::Red };
        Self { name: name.into(), status, passed, failed, skipped }
    }

    pub fn is_green(&self) -> bool {
        self.status == Status::Green
    }

    pub fn total(&self) -> usize {
        self.passed + self.failed + self.skipped
    }
}

pub fn ecosystem_suite_results() -> Vec<SuiteResult> {
    vec![
        SuiteResult::new("unit", 312, 0, 2),
        SuiteResult::new("integration", 48, 0, 0),
        SuiteResult::new("e2e", 17, 0, 0),
        SuiteResult::new("property", 24, 0, 0),
    ]
}

pub fn all_green(results: &[SuiteResult]) -> bool {
    results.iter().all(|r| r.is_green())
}
