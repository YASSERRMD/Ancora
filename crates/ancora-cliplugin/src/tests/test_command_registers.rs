use crate::interface::{
    CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginError, PluginMeta, PluginResult,
};
use crate::registration::PluginRegistry;

struct SimplePlugin {
    meta: PluginMeta,
}

impl SimplePlugin {
    fn new(id: &str, command: &str) -> Self {
        Self {
            meta: PluginMeta::new(id, id, "1.0.0", "test plugin", "test"),
        }
    }
}

// Allow dead code in test helper.
struct RegPlugin {
    meta: PluginMeta,
    cmd: String,
}

impl RegPlugin {
    fn new(id: &str, cmd: &str) -> Self {
        Self {
            meta: PluginMeta::new(id, id, "1.0.0", "test", "test"),
            cmd: cmd.to_string(),
        }
    }
}

impl CliPlugin for RegPlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn commands(&self) -> Vec<CommandSpec> {
        vec![CommandSpec::new(&self.cmd, "short", "long")]
    }

    fn execute(&self, _command: &str, _ctx: ExecContext) -> PluginResult<ExecOutput> {
        Ok(ExecOutput::success(vec!["ok".into()]))
    }
}

#[test]
fn test_plugin_registers_command() {
    let mut registry = PluginRegistry::new();
    let plugin = Box::new(RegPlugin::new("plug.one", "my-cmd"));
    registry
        .register(plugin)
        .expect("registration should succeed");

    assert!(
        registry.has_command("my-cmd"),
        "command should be registered"
    );
    assert_eq!(registry.plugin_count(), 1);
}

#[test]
fn test_duplicate_command_is_rejected() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Box::new(RegPlugin::new("plug.a", "shared-cmd")))
        .unwrap();
    let result = registry.register(Box::new(RegPlugin::new("plug.b", "shared-cmd")));

    assert!(
        matches!(result, Err(PluginError::ConflictingCommand(ref name)) if name == "shared-cmd"),
        "duplicate command should yield ConflictingCommand"
    );
}

#[test]
fn test_multiple_plugins_register_distinct_commands() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Box::new(RegPlugin::new("plug.a", "cmd-a")))
        .unwrap();
    registry
        .register(Box::new(RegPlugin::new("plug.b", "cmd-b")))
        .unwrap();

    assert!(registry.has_command("cmd-a"));
    assert!(registry.has_command("cmd-b"));
    assert_eq!(registry.plugin_count(), 2);
}
