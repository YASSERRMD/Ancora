use std::collections::HashMap;

use crate::interface::{CliPlugin, ExecContext, ExecOutput};
use crate::sample::SamplePlugin;

fn make_ctx(pairs: &[(&str, &str)]) -> ExecContext {
    let mut args = HashMap::new();
    for (k, v) in pairs {
        args.insert(k.to_string(), v.to_string());
    }
    ExecContext::new(args, false)
}

#[test]
fn test_sample_plugin_meta() {
    let plugin = SamplePlugin::new();
    let meta = plugin.meta();
    assert_eq!(meta.id, "ancora.sample");
    assert!(!meta.name.is_empty());
    assert!(!meta.version.is_empty());
}

#[test]
fn test_sample_greet_default_name() {
    let plugin = SamplePlugin::new();
    let ctx = make_ctx(&[]);
    let output = plugin.execute("greet", ctx).expect("greet should succeed");
    assert_eq!(output.exit_code, 0);
    assert!(output.lines.iter().any(|l| l.contains("Hello, World!")));
}

#[test]
fn test_sample_greet_custom_name() {
    let plugin = SamplePlugin::new();
    let ctx = make_ctx(&[("name", "Alice")]);
    let output = plugin.execute("greet", ctx).expect("greet should succeed");
    assert!(output.lines.iter().any(|l| l.contains("Hello, Alice!")));
}

#[test]
fn test_sample_echo_returns_message() {
    let plugin = SamplePlugin::new();
    let ctx = make_ctx(&[("message", "hello world")]);
    let output = plugin.execute("echo", ctx).expect("echo should succeed");
    assert_eq!(output.exit_code, 0);
    assert!(output.lines.iter().any(|l| l == "hello world"));
}

#[test]
fn test_sample_echo_requires_message() {
    let plugin = SamplePlugin::new();
    let ctx = make_ctx(&[]);
    let result = plugin.execute("echo", ctx);
    assert!(result.is_err(), "echo without message should fail");
}

#[test]
fn test_sample_unknown_command_fails() {
    let plugin = SamplePlugin::new();
    let ctx = make_ctx(&[]);
    let result = plugin.execute("nonexistent", ctx);
    assert!(result.is_err(), "unknown command should return an error");
}

#[test]
fn test_sample_registers_two_commands() {
    let plugin = SamplePlugin::new();
    let cmds = plugin.commands();
    assert!(
        cmds.len() >= 2,
        "sample plugin should register at least two commands"
    );
    let names: Vec<&str> = cmds.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"greet"));
    assert!(names.contains(&"echo"));
}
