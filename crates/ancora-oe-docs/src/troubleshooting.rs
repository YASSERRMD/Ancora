//! Troubleshooting guides: known issues, diagnostic steps, and resolutions.

/// Category of troubleshooting issue.
#[derive(Debug, Clone, PartialEq)]
pub enum IssueCategory {
    Tracing,
    Metrics,
    Evaluation,
    CostAccounting,
    Drift,
    Safety,
    Integration,
}

impl std::fmt::Display for IssueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IssueCategory::Tracing => "tracing",
            IssueCategory::Metrics => "metrics",
            IssueCategory::Evaluation => "evaluation",
            IssueCategory::CostAccounting => "cost_accounting",
            IssueCategory::Drift => "drift",
            IssueCategory::Safety => "safety",
            IssueCategory::Integration => "integration",
        };
        write!(f, "{s}")
    }
}

/// A known issue and its resolution steps.
#[derive(Debug, Clone)]
pub struct KnownIssue {
    pub id: String,
    pub category: IssueCategory,
    pub symptom: String,
    pub diagnostic_steps: Vec<String>,
    pub resolution: String,
}

impl KnownIssue {
    pub fn new(
        id: impl Into<String>,
        category: IssueCategory,
        symptom: impl Into<String>,
        resolution: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            category,
            symptom: symptom.into(),
            diagnostic_steps: Vec::new(),
            resolution: resolution.into(),
        }
    }

    pub fn with_step(mut self, step: impl Into<String>) -> Self {
        self.diagnostic_steps.push(step.into());
        self
    }
}

/// Searchable knowledge base of known issues.
#[derive(Debug, Default)]
pub struct TroubleshootingKb {
    issues: Vec<KnownIssue>,
}

impl TroubleshootingKb {
    pub fn add(&mut self, issue: KnownIssue) {
        self.issues.push(issue);
    }

    pub fn search_by_category(&self, category: &IssueCategory) -> Vec<&KnownIssue> {
        self.issues
            .iter()
            .filter(|i| &i.category == category)
            .collect()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&KnownIssue> {
        self.issues.iter().find(|i| i.id == id)
    }

    pub fn total_issues(&self) -> usize {
        self.issues.len()
    }
}
