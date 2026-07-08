use std::collections::HashMap;

use crate::interface::{CliPlugin, CommandSpec, ExecContext, ExecOutput, PluginMeta, PluginResult};
use crate::registration::PluginRegistry;

/// A plugin that records whether its execute method was called.
struct TrackingPlugin {
    meta: PluginMeta,
}

impl TrackingPlugin {
    fn new() -> Self {
        Self {
            meta: PluginMeta::new(
                "tracking",
                "Tracking Plugin",
                "1.0.0",
                "tracks calls",
                "test",
            ),
        }
    }
}

impl CliPlugin for TrackingPlugin {
    fn meta(&self) -> &PluginMeta {
        &self.meta
    }

    fn commands(&self) -> Vec<CommandSpec> {
        vec![CommandSpec::new(
            "run-me",
            "Execute the tracking command",
            "Detailed help",
        )]
    }

    fn execute(&self, command: &str, ctx: ExecContext) -> PluginResult<ExecOutput> {
        let flag = ctx.get_arg("flag").unwrap_or("none");
        Ok(ExecOutput::success(vec![
            format!("executed: {}", command),
            format!("flag: {}", flag),
        ]))
    }
}

#[test]
fn test_dispatch_runs_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(TrackingPlugin::new())).unwrap();

    let mut args = HashMap::new();
    args.insert("flag".to_string(), "active".to_string());
    let ctx = ExecContext::new(args, false);

    let output = registry
        .dispatch("run-me", ctx)
        .expect("dispatch should succeed");

    assert_eq!(output.exit_code, 0);
    assert!(output.lines.iter().any(|l| l.contains("executed: run-me")));
    assert!(output.lines.iter().any(|l| l.contains("flag: active")));
}

#[test]
fn test_dispatch_unknown_command_returns_error() {
    let registry = PluginRegistry::new();
    let ctx = ExecContext::new(HashMap::new(), false);
    let result = registry.dispatch("nonexistent", ctx);

    assert!(result.is_err(), "dispatching unknown command should fail");
}

#[test]
fn test_execution_context_verbose_flag() {
    let ctx = ExecContext::new(HashMap::new(), true);
    assert!(ctx.verbose, "verbose flag should be preserved in context");
}

#[test]
fn test_execution_context_arg_retrieval() {
    let mut args = HashMap::new();
    args.insert("key".to_string(), "value".to_string());
    let ctx = ExecContext::new(args, false);

    assert_eq!(ctx.get_arg("key"), Some("value"));
    assert_eq!(ctx.get_arg("missing"), None);
}
