//! Extensibility overview for the Ancora ecosystem.
//!
//! Describes the core extension points available to plugin authors.

/// Represents an extension point in the Ancora ecosystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionPoint {
    pub id: &'static str,
    pub description: &'static str,
    pub stable: bool,
}

/// Returns all documented extension points.
pub fn all_extension_points() -> Vec<ExtensionPoint> {
    vec![
        ExtensionPoint {
            id: "plugin-sdk",
            description: "Implement custom plugins via the Plugin trait",
            stable: true,
        },
        ExtensionPoint {
            id: "graph-builder",
            description: "Extend the task graph with custom node types",
            stable: true,
        },
        ExtensionPoint {
            id: "fw-adapters",
            description: "Bridge third-party orchestration frameworks",
            stable: false,
        },
        ExtensionPoint {
            id: "cli-plugins",
            description: "Add subcommands to the Ancora CLI",
            stable: true,
        },
    ]
}

/// Returns stable extension points only.
pub fn stable_extension_points() -> Vec<ExtensionPoint> {
    all_extension_points()
        .into_iter()
        .filter(|ep| ep.stable)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_subset_is_smaller() {
        let all = all_extension_points();
        let stable = stable_extension_points();
        assert!(stable.len() < all.len());
    }

    #[test]
    fn all_stable_have_stable_flag() {
        for ep in stable_extension_points() {
            assert!(ep.stable, "expected {} to be stable", ep.id);
        }
    }
}
