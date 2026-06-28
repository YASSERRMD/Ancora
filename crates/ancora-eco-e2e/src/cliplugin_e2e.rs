/// CLI plugin end-to-end: registration and command dispatch.

use std::collections::HashMap;

pub type CommandFn = fn(&[&str]) -> Result<String, String>;

#[derive(Debug, Clone)]
pub struct CliCommand {
    pub name: String,
    pub description: String,
}

impl CliCommand {
    pub fn new(name: &str, description: &str) -> Self {
        CliCommand {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct CliPlugin {
    pub name: String,
    commands: HashMap<String, (CliCommand, CommandFn)>,
}

impl CliPlugin {
    pub fn new(name: &str) -> Self {
        CliPlugin {
            name: name.to_string(),
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, cmd: CliCommand, handler: CommandFn) -> Result<(), String> {
        if cmd.name.is_empty() {
            return Err("command name must not be empty".to_string());
        }
        if self.commands.contains_key(&cmd.name) {
            return Err(format!("command '{}' already registered", cmd.name));
        }
        self.commands.insert(cmd.name.clone(), (cmd, handler));
        Ok(())
    }

    pub fn run(&self, command_name: &str, args: &[&str]) -> Result<String, String> {
        match self.commands.get(command_name) {
            Some((_, handler)) => handler(args),
            None => Err(format!("unknown command: {}", command_name)),
        }
    }

    pub fn list_commands(&self) -> Vec<&CliCommand> {
        let mut cmds: Vec<&CliCommand> = self.commands.values().map(|(c, _)| c).collect();
        cmds.sort_by_key(|c| &c.name);
        cmds
    }

    pub fn command_count(&self) -> usize {
        self.commands.len()
    }
}
