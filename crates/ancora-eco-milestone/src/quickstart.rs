/// Extension author quickstart guide data for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct QuickstartStep {
    pub number: u8,
    pub title: &'static str,
    pub description: &'static str,
    pub command: Option<&'static str>,
}

impl QuickstartStep {
    pub const fn new(
        number: u8,
        title: &'static str,
        description: &'static str,
        command: Option<&'static str>,
    ) -> Self {
        Self {
            number,
            title,
            description,
            command,
        }
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }
}

pub fn quickstart_steps() -> Vec<QuickstartStep> {
    vec![
        QuickstartStep::new(
            1,
            "Install the Ancora CLI",
            "Install the CLI to scaffold and manage plugins",
            Some("cargo install ancora-cli"),
        ),
        QuickstartStep::new(
            2,
            "Scaffold a new plugin",
            "Create a new plugin project from the starter template",
            Some("ancora new-plugin my-plugin"),
        ),
        QuickstartStep::new(
            3,
            "Implement the plugin trait",
            "Open src/lib.rs and implement the Plugin trait",
            None,
        ),
        QuickstartStep::new(
            4,
            "Build and test locally",
            "Verify your plugin builds and passes tests",
            Some("cargo test"),
        ),
        QuickstartStep::new(
            5,
            "Publish to the registry",
            "Publish your plugin to the Ancora registry",
            Some("ancora publish"),
        ),
    ]
}

pub fn step_count() -> usize {
    quickstart_steps().len()
}
