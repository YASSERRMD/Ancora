use crate::entry::{AuditEntry, Outcome, Severity};

#[derive(Debug, Default)]
pub struct AuditStats {
    pub total: usize,
    pub successes: usize,
    pub failures: usize,
    pub blocked: usize,
    pub critical: usize,
    pub errors: usize,
}

impl AuditStats {
    pub fn from_entries<'a>(entries: impl Iterator<Item = &'a AuditEntry>) -> Self {
        let mut stats = Self::default();
        for e in entries {
            stats.total += 1;
            match e.outcome {
                Outcome::Success => stats.successes += 1,
                Outcome::Failure => stats.failures += 1,
                Outcome::Blocked => stats.blocked += 1,
            }
            match e.severity {
                Severity::Critical => stats.critical += 1,
                Severity::Error => stats.errors += 1,
                _ => {}
            }
        }
        stats
    }

    pub fn failure_rate(&self) -> f64 {
        if self.total == 0 { 0.0 } else { self.failures as f64 / self.total as f64 }
    }
}
