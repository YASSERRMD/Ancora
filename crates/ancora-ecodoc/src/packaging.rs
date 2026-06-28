//! Packaging templates and release checklist for Ancora plugins.
//!
//! Guides plugin authors through the steps required to publish
//! a plugin to crates.io and the Ancora catalog.

/// A single step in the release checklist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecklistStep {
    pub id: &'static str,
    pub description: &'static str,
    pub mandatory: bool,
}

/// The full plugin release checklist.
pub fn release_checklist() -> Vec<ChecklistStep> {
    vec![
        ChecklistStep {
            id: "bump-version",
            description: "Update version in Cargo.toml following SemVer",
            mandatory: true,
        },
        ChecklistStep {
            id: "update-changelog",
            description: "Add an entry to CHANGELOG.md",
            mandatory: true,
        },
        ChecklistStep {
            id: "run-tests",
            description: "Ensure `cargo test` passes with no failures",
            mandatory: true,
        },
        ChecklistStep {
            id: "check-msrv",
            description: "Verify the crate compiles on the stated MSRV",
            mandatory: true,
        },
        ChecklistStep {
            id: "catalog-entry",
            description: "Update ancora-catalog.toml with the new version",
            mandatory: true,
        },
        ChecklistStep {
            id: "docs-preview",
            description: "Preview docs with `cargo doc --open`",
            mandatory: false,
        },
        ChecklistStep {
            id: "publish-dry-run",
            description: "Run `cargo publish --dry-run` to catch packaging issues",
            mandatory: false,
        },
        ChecklistStep {
            id: "tag-release",
            description: "Create a git tag matching the crate version",
            mandatory: true,
        },
    ]
}

/// Returns only the mandatory steps.
pub fn mandatory_steps() -> Vec<ChecklistStep> {
    release_checklist()
        .into_iter()
        .filter(|s| s.mandatory)
        .collect()
}

/// A simple packaging manifest used in documentation examples.
#[derive(Debug, Clone)]
pub struct PackagingManifest {
    pub crate_name: String,
    pub version: String,
    pub catalog_name: String,
}

impl PackagingManifest {
    pub fn new(
        crate_name: impl Into<String>,
        version: impl Into<String>,
        catalog_name: impl Into<String>,
    ) -> Self {
        Self {
            crate_name: crate_name.into(),
            version: version.into(),
            catalog_name: catalog_name.into(),
        }
    }

    /// Validate that names and version are non-empty.
    pub fn validate(&self) -> Result<(), String> {
        if self.crate_name.is_empty() {
            return Err("crate_name is required".into());
        }
        if self.version.is_empty() {
            return Err("version is required".into());
        }
        if self.catalog_name.is_empty() {
            return Err("catalog_name is required".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_checklist_has_mandatory_steps() {
        let steps = mandatory_steps();
        assert!(!steps.is_empty());
        assert!(steps.iter().all(|s| s.mandatory));
    }

    #[test]
    fn run_tests_step_is_mandatory() {
        assert!(mandatory_steps().iter().any(|s| s.id == "run-tests"));
    }

    #[test]
    fn valid_manifest_passes() {
        let m = PackagingManifest::new("my-crate", "0.1.0", "my-plugin");
        assert!(m.validate().is_ok());
    }

    #[test]
    fn empty_crate_name_fails() {
        let m = PackagingManifest::new("", "0.1.0", "my-plugin");
        assert!(m.validate().is_err());
    }
}
