//! App gallery index.
//!
//! Lists all available sample applications with metadata so that
//! tooling can discover and launch them programmatically.

#[derive(Debug, Clone, PartialEq)]
pub enum AppCategory {
    DocumentProcessing,
    Research,
    Coding,
    DataAnalysis,
    Support,
    Compliance,
}

impl std::fmt::Display for AppCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AppCategory::DocumentProcessing => "Document Processing",
            AppCategory::Research => "Research",
            AppCategory::Coding => "Coding",
            AppCategory::DataAnalysis => "Data Analysis",
            AppCategory::Support => "Support",
            AppCategory::Compliance => "Compliance",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: AppCategory,
    pub offline_capable: bool,
    pub air_gapped_capable: bool,
}

impl AppEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        category: AppCategory,
        offline_capable: bool,
        air_gapped_capable: bool,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            category,
            offline_capable,
            air_gapped_capable,
        }
    }
}

/// The full static gallery of sample applications.
pub fn gallery() -> Vec<AppEntry> {
    vec![
        AppEntry::new(
            "document-qa",
            "Document QA",
            "Ask questions over a local document corpus.",
            AppCategory::DocumentProcessing,
            true,
            true,
        ),
        AppEntry::new(
            "research-assistant",
            "Research Assistant",
            "Synthesise summaries from a local knowledge base.",
            AppCategory::Research,
            true,
            true,
        ),
        AppEntry::new(
            "coding-assistant",
            "Coding Assistant",
            "Look up code snippets and generate stubs offline.",
            AppCategory::Coding,
            true,
            true,
        ),
        AppEntry::new(
            "data-analysis",
            "Data Analysis",
            "Compute statistics over in-memory datasets.",
            AppCategory::DataAnalysis,
            true,
            true,
        ),
        AppEntry::new(
            "customer-support",
            "Customer Support",
            "Route support tickets via a local template engine.",
            AppCategory::Support,
            true,
            false,
        ),
        AppEntry::new(
            "compliance-review",
            "Compliance Review (Government)",
            "Evaluate artifacts against government compliance rules - air-gapped.",
            AppCategory::Compliance,
            true,
            true,
        ),
    ]
}

/// Return only apps that work in air-gapped environments.
pub fn air_gapped_apps() -> Vec<AppEntry> {
    gallery()
        .into_iter()
        .filter(|a| a.air_gapped_capable)
        .collect()
}

/// Return only apps that work fully offline.
pub fn offline_apps() -> Vec<AppEntry> {
    gallery()
        .into_iter()
        .filter(|a| a.offline_capable)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_is_non_empty() {
        assert!(!gallery().is_empty());
    }

    #[test]
    fn all_apps_are_offline_capable() {
        let not_offline: Vec<_> = gallery()
            .into_iter()
            .filter(|a| !a.offline_capable)
            .collect();
        assert!(
            not_offline.is_empty(),
            "apps not offline-capable: {:?}",
            not_offline.iter().map(|a| &a.id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn compliance_app_is_air_gapped() {
        let apps = air_gapped_apps();
        assert!(apps.iter().any(|a| a.id == "compliance-review"));
    }
}
