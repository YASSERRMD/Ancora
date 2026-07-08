/// Upgrade notes for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct UpgradeNote {
    pub from_version: &'static str,
    pub to_version: &'static str,
    pub breaking: bool,
    pub steps: Vec<&'static str>,
}

impl UpgradeNote {
    pub fn new(
        from_version: &'static str,
        to_version: &'static str,
        breaking: bool,
        steps: Vec<&'static str>,
    ) -> Self {
        Self {
            from_version,
            to_version,
            breaking,
            steps,
        }
    }

    pub fn is_breaking(&self) -> bool {
        self.breaking
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

pub fn upgrade_notes() -> Vec<UpgradeNote> {
    vec![
        UpgradeNote::new(
            "0.5.x",
            "0.6.0",
            true,
            vec![
                "Update plugin manifest to schema v3",
                "Rename `PluginCtx::invoke` to `PluginCtx::call`",
                "Remove deprecated `ancora_plugin::v1` imports",
                "Re-run `cargo build` and resolve any type errors",
            ],
        ),
        UpgradeNote::new(
            "0.6.0",
            "0.6.x",
            false,
            vec!["Run `cargo update` to pick up patch releases"],
        ),
    ]
}
