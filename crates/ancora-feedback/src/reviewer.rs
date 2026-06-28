use std::collections::HashMap;

/// A reviewer who can be assigned to review queued runs.
#[derive(Debug, Clone)]
pub struct Reviewer {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// Assignment of a reviewer to a queued run.
#[derive(Debug, Clone)]
pub struct Assignment {
    pub run_id: String,
    pub reviewer_id: String,
}

/// Registry that manages reviewer assignment.
#[derive(Debug, Default)]
pub struct ReviewerRegistry {
    reviewers: HashMap<String, Reviewer>,
    assignments: Vec<Assignment>,
}

impl ReviewerRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a reviewer.
    pub fn register(&mut self, reviewer: Reviewer) {
        self.reviewers.insert(reviewer.id.clone(), reviewer);
    }

    /// Assign a reviewer to a run. Returns an error if the reviewer is not found.
    pub fn assign(&mut self, run_id: impl Into<String>, reviewer_id: &str) -> Result<(), String> {
        if self.reviewers.contains_key(reviewer_id) {
            self.assignments.push(Assignment {
                run_id: run_id.into(),
                reviewer_id: reviewer_id.to_string(),
            });
            Ok(())
        } else {
            Err(format!("Reviewer '{}' not found", reviewer_id))
        }
    }

    /// Look up the reviewer assigned to a run.
    pub fn assigned_reviewer(&self, run_id: &str) -> Option<&Reviewer> {
        self.assignments
            .iter()
            .rev()
            .find(|a| a.run_id == run_id)
            .and_then(|a| self.reviewers.get(&a.reviewer_id))
    }

    /// List all assignments.
    pub fn assignments(&self) -> &[Assignment] {
        &self.assignments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_reviewer_to_run() {
        let mut reg = ReviewerRegistry::new();
        reg.register(Reviewer {
            id: "r1".into(),
            name: "Alice".into(),
            email: "alice@example.com".into(),
        });
        reg.assign("run-1", "r1").unwrap();
        let rev = reg.assigned_reviewer("run-1").unwrap();
        assert_eq!(rev.name, "Alice");
    }

    #[test]
    fn unknown_reviewer_errors() {
        let mut reg = ReviewerRegistry::new();
        let result = reg.assign("run-1", "ghost");
        assert!(result.is_err());
    }
}
