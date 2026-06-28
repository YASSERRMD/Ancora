//! Extension author quickstart for the Ancora ecosystem.
//!
//! Walks new contributors through the minimum set of steps needed
//! to publish a working plugin.

/// A step in the quickstart guide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickstartStep {
    pub order: u32,
    pub title: &'static str,
    pub command: Option<&'static str>,
    pub description: &'static str,
}

/// Returns the ordered quickstart steps for a new plugin author.
pub fn quickstart_steps() -> Vec<QuickstartStep> {
    vec![
        QuickstartStep {
            order: 1,
            title: "Create a new crate",
            command: Some("cargo new --lib my-ancora-plugin"),
            description: "Scaffold a new library crate for your plugin",
        },
        QuickstartStep {
            order: 2,
            title: "Add Ancora SDK dependency",
            command: None,
            description: "Add `ancora-sdk = \"0.1\"` to your Cargo.toml dependencies",
        },
        QuickstartStep {
            order: 3,
            title: "Implement the Plugin trait",
            command: None,
            description: "Define a struct and implement `Plugin` from ancora-sdk",
        },
        QuickstartStep {
            order: 4,
            title: "Write tests",
            command: Some("cargo test"),
            description: "Cover the happy path and at least two error cases",
        },
        QuickstartStep {
            order: 5,
            title: "Add catalog entry",
            command: None,
            description: "Create an `ancora-catalog.toml` in your crate root",
        },
        QuickstartStep {
            order: 6,
            title: "Publish to crates.io",
            command: Some("cargo publish"),
            description: "Publish the crate so it can be discovered by Ancora users",
        },
    ]
}

/// Returns the steps that have an associated command.
pub fn steps_with_commands() -> Vec<QuickstartStep> {
    quickstart_steps()
        .into_iter()
        .filter(|s| s.command.is_some())
        .collect()
}

/// Returns step by order number, or `None` if not found.
pub fn step_by_order(order: u32) -> Option<QuickstartStep> {
    quickstart_steps().into_iter().find(|s| s.order == order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_are_ordered() {
        let steps = quickstart_steps();
        for (i, step) in steps.iter().enumerate() {
            assert_eq!(step.order as usize, i + 1);
        }
    }

    #[test]
    fn first_step_creates_crate() {
        let step = step_by_order(1).unwrap();
        assert!(step.command.unwrap().contains("cargo new"));
    }

    #[test]
    fn steps_with_commands_is_subset() {
        let all = quickstart_steps();
        let with_cmd = steps_with_commands();
        assert!(with_cmd.len() <= all.len());
    }

    #[test]
    fn step_out_of_range_returns_none() {
        assert!(step_by_order(999).is_none());
    }
}
