/// Pass/fail report produced by the kit runner.

/// Overall status of a kit run.
#[derive(Debug, Clone, PartialEq)]
pub enum KitStatus {
    AllPassed,
    SomeFailed { failed: usize, total: usize },
}

impl KitStatus {
    pub fn is_pass(&self) -> bool {
        matches!(self, KitStatus::AllPassed)
    }
}

impl std::fmt::Display for KitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KitStatus::AllPassed => write!(f, "ALL PASSED"),
            KitStatus::SomeFailed { failed, total } => {
                write!(f, "{failed}/{total} FAILED")
            }
        }
    }
}

/// The complete report from one or more kit runs.
#[derive(Debug, Clone)]
pub struct KitReport {
    pub title: String,
    pub status: KitStatus,
    pub lines: Vec<String>,
    pub total: usize,
    pub passed: usize,
}

impl KitReport {
    /// Format as a human-readable text block.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("=== {} ===\n", self.title));
        out.push_str(&format!("Status: {}\n", self.status));
        out.push_str(&format!("Passed: {}/{}\n\n", self.passed, self.total));
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    /// Return true only if every check passed.
    pub fn is_pass(&self) -> bool {
        self.status.is_pass()
    }

    /// Return the list of failing lines.
    pub fn failing_lines(&self) -> Vec<&str> {
        self.lines
            .iter()
            .filter(|l| l.starts_with("[FAIL]"))
            .map(|l| l.as_str())
            .collect()
    }
}
