//! Troubleshooting guide for Ancora extension authors.
//!
//! Provides a structured database of known issues and their resolutions.

/// Severity of a known issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        };
        write!(f, "{label}")
    }
}

/// A known issue entry in the troubleshooting database.
#[derive(Debug, Clone)]
pub struct KnownIssue {
    pub id: &'static str,
    pub severity: IssueSeverity,
    pub symptom: &'static str,
    pub cause: &'static str,
    pub resolution: &'static str,
}

/// Returns the complete troubleshooting database.
pub fn known_issues() -> Vec<KnownIssue> {
    vec![
        KnownIssue {
            id: "trbl-001",
            severity: IssueSeverity::Error,
            symptom: "Plugin fails to load with `symbol not found`",
            cause: "ABI mismatch between the plugin and the host runtime",
            resolution: "Recompile the plugin against the same Ancora version as the host",
        },
        KnownIssue {
            id: "trbl-002",
            severity: IssueSeverity::Warning,
            symptom: "Capability request silently denied",
            cause: "Plugin trust level does not permit the requested capability",
            resolution: "Raise the plugin trust level or remove the capability request",
        },
        KnownIssue {
            id: "trbl-003",
            severity: IssueSeverity::Error,
            symptom: "Graph build panics with `cycle detected`",
            cause: "The task graph contains a cycle",
            resolution: "Use `TaskGraph::has_cycle()` to detect cycles before running",
        },
        KnownIssue {
            id: "trbl-004",
            severity: IssueSeverity::Info,
            symptom: "CLI command not visible in `ancora help`",
            cause: "The plugin was registered after the CLI registry was frozen",
            resolution: "Register CLI commands in the plugin `Init` event handler",
        },
    ]
}

/// Search for known issues matching a keyword in the symptom or cause.
pub fn search(keyword: &str) -> Vec<KnownIssue> {
    let kw = keyword.to_lowercase();
    known_issues()
        .into_iter()
        .filter(|i| {
            i.symptom.to_lowercase().contains(&kw)
                || i.cause.to_lowercase().contains(&kw)
                || i.resolution.to_lowercase().contains(&kw)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_issues_non_empty() {
        assert!(!known_issues().is_empty());
    }

    #[test]
    fn search_finds_cycle_issue() {
        let results = search("cycle");
        assert!(results.iter().any(|i| i.id == "trbl-003"));
    }

    #[test]
    fn search_no_match_returns_empty() {
        let results = search("xyzzy-no-match");
        assert!(results.is_empty());
    }

    #[test]
    fn severity_ordering() {
        assert!(IssueSeverity::Critical > IssueSeverity::Error);
        assert!(IssueSeverity::Error > IssueSeverity::Warning);
    }
}
