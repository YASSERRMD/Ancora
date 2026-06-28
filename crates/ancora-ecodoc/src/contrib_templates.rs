//! Contribution templates for Ancora plugin authors.
//!
//! Provides standard template files that new plugin crates should include
//! to ensure consistency across the ecosystem.

/// The kind of contribution template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateKind {
    Readme,
    Contributing,
    ChangeLog,
    SecurityPolicy,
    IssueTemplate,
    PullRequestTemplate,
}

impl TemplateKind {
    /// Returns the recommended filename for this template.
    pub fn filename(&self) -> &'static str {
        match self {
            Self::Readme => "README.md",
            Self::Contributing => "CONTRIBUTING.md",
            Self::ChangeLog => "CHANGELOG.md",
            Self::SecurityPolicy => "SECURITY.md",
            Self::IssueTemplate => ".github/ISSUE_TEMPLATE.md",
            Self::PullRequestTemplate => ".github/PULL_REQUEST_TEMPLATE.md",
        }
    }

    /// Returns a short description of the template's purpose.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Readme => "Project overview and usage guide",
            Self::Contributing => "Contribution guidelines",
            Self::ChangeLog => "Version history",
            Self::SecurityPolicy => "Vulnerability reporting instructions",
            Self::IssueTemplate => "Standardised issue submission form",
            Self::PullRequestTemplate => "Pull-request checklist",
        }
    }
}

/// Returns all mandatory templates that a plugin crate must include.
pub fn mandatory_templates() -> Vec<TemplateKind> {
    vec![
        TemplateKind::Readme,
        TemplateKind::Contributing,
        TemplateKind::ChangeLog,
        TemplateKind::SecurityPolicy,
    ]
}

/// Returns all recommended (non-mandatory) templates.
pub fn recommended_templates() -> Vec<TemplateKind> {
    vec![
        TemplateKind::IssueTemplate,
        TemplateKind::PullRequestTemplate,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_templates_have_filenames() {
        for kind in mandatory_templates()
            .iter()
            .chain(recommended_templates().iter())
        {
            assert!(!kind.filename().is_empty());
        }
    }

    #[test]
    fn readme_is_mandatory() {
        assert!(mandatory_templates().contains(&TemplateKind::Readme));
    }

    #[test]
    fn issue_template_is_recommended_not_mandatory() {
        assert!(!mandatory_templates().contains(&TemplateKind::IssueTemplate));
        assert!(recommended_templates().contains(&TemplateKind::IssueTemplate));
    }
}
