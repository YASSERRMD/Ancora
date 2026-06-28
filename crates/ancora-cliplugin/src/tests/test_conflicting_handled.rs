use std::collections::HashMap;

use crate::interface::{CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginError, PluginMeta, PluginResult};
use crate::registration::PluginRegistry;

struct AliasPlugin {
    meta: PluginMeta,
    cmd: String,
    alias: String,
}

impl AliasPlugin {
    fn new(id: &str, cmd: &str, alias: &str) -> Self {
        Self {
            meta: PluginMeta::new(id, id, "1.0.0", "test", "test"),
            cmd: cmd.to_string(),
            alias: alias.to_string(),
        }
    }
}

impl CliPlugin for AliasPlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn commands(&self) -> Vec<CommandSpec> {
        vec![CommandSpec::new(&self.cmd, "short", "long").with_alias(&self.alias)]
    }

    fn execute(&self, _cmd: &str, _ctx: ExecContext) -> PluginResult<ExecOutput> {
        Ok(ExecOutput::success(vec!["ok".into()]))
    }
}

#[test]
fn test_alias_conflict_is_rejected() {
    let mut registry = PluginRegistry::new();
    // First plugin registers "foo" with alias "bar".
    registry
        .register(Box::new(AliasPlugin::new("plug.a", "foo", "bar")))
        .unwrap();

    // Second plugin tries to register a command also named "bar" (the alias above).
    let result = registry.register(Box::new(AliasPlugin::new("plug.b", "bar", "baz")));

    assert!(
        matches!(result, Err(PluginError::ConflictingCommand(ref n)) if n == "bar"),
        "alias collision should be detected"
    );
}

#[test]
fn test_canonical_name_conflict_rejected() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Box::new(AliasPlugin::new("plug.a", "cmd-x", "cx")))
        .unwrap();

    let result = registry.register(Box::new(AliasPlugin::new("plug.b", "cmd-x", "different")));
    assert!(
        matches!(result, Err(PluginError::ConflictingCommand(_))),
        "duplicate canonical name should be rejected"
    );
}

#[test]
fn test_non_conflicting_plugins_both_work() {
    let mut registry = PluginRegistry::new();
    registry
        .register(Box::new(AliasPlugin::new("plug.a", "alpha", "a")))
        .unwrap();
    registry
        .register(Box::new(AliasPlugin::new("plug.b", "beta", "b")))
        .unwrap();

    let ctx = ExecContext::new(HashMap::new(), false);
    assert!(registry.dispatch("alpha", ctx.clone()).is_ok());
    assert!(registry.dispatch("beta", ctx.clone()).is_ok());
    // Also via aliases.
    assert!(registry.dispatch("a", ctx.clone()).is_ok());
    assert!(registry.dispatch("b", ctx).is_ok());
}

#[test]
fn test_conflicting_plugin_not_partially_registered() {
    let mut registry = PluginRegistry::new();

    struct TwoCmdPlugin {
        meta: PluginMeta,
    }
    impl CliPlugin for TwoCmdPlugin {
        fn meta(&self) -> &PluginMeta { &self.meta }
        fn commands(&self) -> Vec<CommandSpec> {
            vec![
                CommandSpec::new("unique-cmd", "short", "long"),
                CommandSpec::new("conflict-cmd", "short", "long"),
            ]
        }
        fn execute(&self, _: &str, _: ExecContext) -> PluginResult<ExecOutput> {
            Ok(ExecOutput::success(vec![]))
        }
    }

    // Pre-register "conflict-cmd".
    registry.register(Box::new(AliasPlugin::new("pre", "conflict-cmd", "pre-alias"))).unwrap();

    let conflicting = Box::new(TwoCmdPlugin {
        meta: PluginMeta::new("plug.conflict", "Conflict", "1.0.0", "test", "test"),
    });

    let result = registry.register(conflicting);
    assert!(result.is_err());

    // "unique-cmd" should NOT be registered because the whole registration rolled back.
    assert!(
        !registry.has_command("unique-cmd"),
        "partial registration should not occur on conflict"
    );
}
