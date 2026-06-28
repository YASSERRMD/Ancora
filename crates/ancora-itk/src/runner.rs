/// Kit runner - orchestrates multiple kit runs and collects results.

use crate::report::{KitReport, KitStatus};

/// A named check result from any kit.
#[derive(Debug, Clone)]
pub struct RunResult {
    pub kit_name: String,
    pub check_name: String,
    pub passed: bool,
    pub message: String,
}

/// Collects results from multiple kit runs.
#[derive(Debug, Default)]
pub struct Runner {
    results: Vec<RunResult>,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            results: Vec::new(),
        }
    }

    /// Record a batch of check results from a named kit.
    pub fn record_kit(
        &mut self,
        kit_name: impl Into<String>,
        checks: impl IntoIterator<Item = (String, bool, String)>,
    ) {
        let kit_name = kit_name.into();
        for (check_name, passed, message) in checks {
            self.results.push(RunResult {
                kit_name: kit_name.clone(),
                check_name,
                passed,
                message,
            });
        }
    }

    /// Return the total number of checks recorded.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Return the number of passing checks.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Return the number of failing checks.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Build a [`KitReport`] from the collected results.
    pub fn into_report(self, title: impl Into<String>) -> KitReport {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let status = if passed == total {
            KitStatus::AllPassed
        } else {
            KitStatus::SomeFailed {
                failed: total - passed,
                total,
            }
        };

        let lines: Vec<String> = self
            .results
            .iter()
            .map(|r| {
                let mark = if r.passed { "PASS" } else { "FAIL" };
                format!("[{mark}] {}::{} - {}", r.kit_name, r.check_name, r.message)
            })
            .collect();

        KitReport {
            title: title.into(),
            status,
            lines,
            total,
            passed,
        }
    }

    /// Borrow the raw results.
    pub fn results(&self) -> &[RunResult] {
        &self.results
    }
}
