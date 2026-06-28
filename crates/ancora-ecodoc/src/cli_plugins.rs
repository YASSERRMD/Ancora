//! CLI plugin integration for the Ancora command-line interface.
//!
//! Describes how third-party plugins can register additional subcommands
//! that appear in `ancora help`.

/// A CLI subcommand contributed by a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommand {
    pub name: String,
    pub about: String,
    pub usage: String,
}

/// A registry of CLI commands contributed by plugins.
#[derive(Debug, Default)]
pub struct CliRegistry {
    commands: Vec<CliCommand>,
}

impl CliRegistry {
    /// Create an empty CLI registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new command. Returns an error if the name is already taken.
    pub fn add(&mut self, cmd: CliCommand) -> Result<(), String> {
        if cmd.name.is_empty() {
            return Err("command name must not be empty".into());
        }
        if self.commands.iter().any(|c| c.name == cmd.name) {
            return Err(format!("command '{}' already registered", cmd.name));
        }
        self.commands.push(cmd);
        Ok(())
    }

    /// Find a command by name.
    pub fn find(&self, name: &str) -> Option<&CliCommand> {
        self.commands.iter().find(|c| c.name == name)
    }

    /// List all registered commands sorted by name.
    pub fn list(&self) -> Vec<&CliCommand> {
        let mut cmds: Vec<&CliCommand> = self.commands.iter().collect();
        cmds.sort_by_key(|c| c.name.as_str());
        cmds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd(name: &str) -> CliCommand {
        CliCommand {
            name: name.into(),
            about: "does a thing".into(),
            usage: format!("ancora {name} [OPTIONS]"),
        }
    }

    #[test]
    fn add_and_find() {
        let mut reg = CliRegistry::new();
        reg.add(cmd("deploy")).unwrap();
        assert!(reg.find("deploy").is_some());
    }

    #[test]
    fn duplicate_name_is_rejected() {
        let mut reg = CliRegistry::new();
        reg.add(cmd("run")).unwrap();
        assert!(reg.add(cmd("run")).is_err());
    }

    #[test]
    fn list_is_sorted() {
        let mut reg = CliRegistry::new();
        reg.add(cmd("zoo")).unwrap();
        reg.add(cmd("apple")).unwrap();
        let names: Vec<&str> = reg.list().iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["apple", "zoo"]);
    }
}
