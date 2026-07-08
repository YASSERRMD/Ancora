//! Run diff view - compare two runs step-by-step.

#[derive(Debug, Clone, PartialEq)]
pub enum DiffKind {
    Same,
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone)]
pub struct StepDiff {
    pub step_index: usize,
    pub kind: DiffKind,
    pub label_a: Option<String>,
    pub label_b: Option<String>,
    pub cost_a: Option<f64>,
    pub cost_b: Option<f64>,
    pub tokens_in_a: Option<u32>,
    pub tokens_in_b: Option<u32>,
    pub tokens_out_a: Option<u32>,
    pub tokens_out_b: Option<u32>,
}

impl StepDiff {
    pub fn cost_delta(&self) -> f64 {
        let a = self.cost_a.unwrap_or(0.0);
        let b = self.cost_b.unwrap_or(0.0);
        b - a
    }

    pub fn is_significant(&self) -> bool {
        self.kind != DiffKind::Same
    }
}

pub struct RunDiff {
    pub run_id_a: String,
    pub run_id_b: String,
    pub diffs: Vec<StepDiff>,
}

impl RunDiff {
    pub fn new(
        run_id_a: impl Into<String>,
        run_id_b: impl Into<String>,
        diffs: Vec<StepDiff>,
    ) -> Self {
        Self {
            run_id_a: run_id_a.into(),
            run_id_b: run_id_b.into(),
            diffs,
        }
    }

    pub fn changed_steps(&self) -> Vec<&StepDiff> {
        self.diffs.iter().filter(|d| d.is_significant()).collect()
    }

    pub fn total_cost_delta(&self) -> f64 {
        self.diffs.iter().map(|d| d.cost_delta()).sum()
    }

    pub fn summary(&self) -> DiffSummary {
        let mut added = 0;
        let mut removed = 0;
        let mut changed = 0;
        let mut same = 0;
        for d in &self.diffs {
            match d.kind {
                DiffKind::Added => added += 1,
                DiffKind::Removed => removed += 1,
                DiffKind::Changed => changed += 1,
                DiffKind::Same => same += 1,
            }
        }
        DiffSummary {
            added,
            removed,
            changed,
            same,
        }
    }
}

#[derive(Debug)]
pub struct DiffSummary {
    pub added: usize,
    pub removed: usize,
    pub changed: usize,
    pub same: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_summary() {
        let diff = RunDiff::new(
            "r1",
            "r2",
            vec![
                StepDiff {
                    step_index: 0,
                    kind: DiffKind::Same,
                    label_a: Some("a".into()),
                    label_b: Some("a".into()),
                    cost_a: Some(0.01),
                    cost_b: Some(0.01),
                    tokens_in_a: None,
                    tokens_in_b: None,
                    tokens_out_a: None,
                    tokens_out_b: None,
                },
                StepDiff {
                    step_index: 1,
                    kind: DiffKind::Changed,
                    label_a: Some("b".into()),
                    label_b: Some("b2".into()),
                    cost_a: Some(0.01),
                    cost_b: Some(0.02),
                    tokens_in_a: None,
                    tokens_in_b: None,
                    tokens_out_a: None,
                    tokens_out_b: None,
                },
            ],
        );
        let summary = diff.summary();
        assert_eq!(summary.same, 1);
        assert_eq!(summary.changed, 1);
        assert!((diff.total_cost_delta() - 0.01).abs() < 1e-9);
    }
}
