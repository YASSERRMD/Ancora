/// Severity level of a known limitation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
        };
        write!(f, "{}", s)
    }
}

/// A documented limitation of the observability/eval stack.
#[derive(Debug, Clone)]
pub struct Limitation {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub description: String,
    pub workaround: Option<String>,
    pub tracking_issue: Option<String>,
}

impl Limitation {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            severity,
            description: description.into(),
            workaround: None,
            tracking_issue: None,
        }
    }

    pub fn with_workaround(mut self, w: impl Into<String>) -> Self {
        self.workaround = Some(w.into());
        self
    }

    pub fn with_issue(mut self, issue: impl Into<String>) -> Self {
        self.tracking_issue = Some(issue.into());
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!(
            "[{}] {} (severity: {})\n  {}\n",
            self.id, self.title, self.severity, self.description
        );
        if let Some(w) = &self.workaround {
            out.push_str(&format!("  Workaround: {}\n", w));
        }
        if let Some(i) = &self.tracking_issue {
            out.push_str(&format!("  Issue: {}\n", i));
        }
        out
    }
}

/// Return limitations at or above the given severity.
pub fn filter_by_severity<'a>(items: &'a [Limitation], min: &Severity) -> Vec<&'a Limitation> {
    items.iter().filter(|l| &l.severity >= min).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limitation_renders_correctly() {
        let l = Limitation::new(
            "LIM-001",
            "High-cardinality label explosion",
            Severity::High,
            "Metrics with >10k label combinations may exceed backend limits.",
        )
        .with_workaround("Use label allow-list to cap cardinality.");
        let r = l.render();
        assert!(r.contains("LIM-001"));
        assert!(r.contains("high"));
        assert!(r.contains("Workaround"));
    }

    #[test]
    fn severity_ordering() {
        assert!(Severity::High > Severity::Low);
        assert!(Severity::Medium >= Severity::Medium);
    }

    #[test]
    fn filter_by_severity_works() {
        let items = vec![
            Limitation::new("L1", "minor", Severity::Low, "desc"),
            Limitation::new("L2", "major", Severity::High, "desc"),
        ];
        let high = filter_by_severity(&items, &Severity::High);
        assert_eq!(high.len(), 1);
        assert_eq!(high[0].id, "L2");
    }
}
