//! Run list - browse, filter, and search local agent runs.

#[derive(Debug, Clone, PartialEq)]
pub enum RunStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for RunStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunStatus::Running => write!(f, "running"),
            RunStatus::Completed => write!(f, "completed"),
            RunStatus::Failed => write!(f, "failed"),
            RunStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunSummary {
    pub id: String,
    pub label: String,
    pub status: RunStatus,
    pub started_at: u64,
    pub duration_ms: Option<u64>,
    pub total_cost_usd: Option<f64>,
    pub step_count: usize,
    pub tags: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct RunListFilter {
    pub query: String,
    pub status: Option<RunStatus>,
    pub tag: Option<String>,
}

pub struct RunList {
    runs: Vec<RunSummary>,
}

impl RunList {
    pub fn new(runs: Vec<RunSummary>) -> Self {
        Self { runs }
    }

    pub fn filter(&self, f: &RunListFilter) -> Vec<&RunSummary> {
        self.runs
            .iter()
            .filter(|r| {
                let query_match = f.query.is_empty()
                    || r.label.to_lowercase().contains(&f.query.to_lowercase())
                    || r.id.contains(&f.query);
                let status_match = f.status.as_ref().is_none_or(|s| &r.status == s);
                let tag_match = f.tag.as_ref().is_none_or(|t| r.tags.contains(t));
                query_match && status_match && tag_match
            })
            .collect()
    }

    pub fn all(&self) -> &[RunSummary] {
        &self.runs
    }

    pub fn get(&self, id: &str) -> Option<&RunSummary> {
        self.runs.iter().find(|r| r.id == id)
    }

    pub fn sorted_by_started(&self) -> Vec<&RunSummary> {
        let mut refs: Vec<&RunSummary> = self.runs.iter().collect();
        refs.sort_by_key(|b| std::cmp::Reverse(b.started_at));
        refs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_runs() -> RunList {
        RunList::new(vec![
            RunSummary {
                id: "r1".into(),
                label: "Alpha run".into(),
                status: RunStatus::Completed,
                started_at: 1000,
                duration_ms: Some(500),
                total_cost_usd: Some(0.01),
                step_count: 3,
                tags: vec!["prod".into()],
            },
            RunSummary {
                id: "r2".into(),
                label: "Beta run".into(),
                status: RunStatus::Failed,
                started_at: 2000,
                duration_ms: None,
                total_cost_usd: None,
                step_count: 1,
                tags: vec!["dev".into()],
            },
        ])
    }

    #[test]
    fn test_filter_by_query() {
        let list = sample_runs();
        let f = RunListFilter {
            query: "alpha".into(),
            ..Default::default()
        };
        let results = list.filter(&f);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "r1");
    }

    #[test]
    fn test_filter_by_status() {
        let list = sample_runs();
        let f = RunListFilter {
            status: Some(RunStatus::Failed),
            ..Default::default()
        };
        let results = list.filter(&f);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "r2");
    }

    #[test]
    fn test_sorted_by_started() {
        let list = sample_runs();
        let sorted = list.sorted_by_started();
        assert_eq!(sorted[0].id, "r2");
        assert_eq!(sorted[1].id, "r1");
    }
}
