use crate::cliplugin_e2e::{CliCommand, CliPlugin};

fn echo_handler(args: &[&str]) -> Result<String, String> {
    Ok(args.join(" "))
}

fn fail_handler(_args: &[&str]) -> Result<String, String> {
    Err("intentional failure".to_string())
}

#[test]
fn test_cli_plugin_registers_and_runs() {
    let mut plugin = CliPlugin::new("my-cli");
    let cmd = CliCommand::new("echo", "Echoes arguments back");
    plugin
        .register(cmd, echo_handler)
        .expect("register must succeed");
    let result = plugin
        .run("echo", &["hello", "world"])
        .expect("run must succeed");
    assert_eq!(result, "hello world");
}

#[test]
fn test_cli_plugin_unknown_command() {
    let plugin = CliPlugin::new("bare-cli");
    let result = plugin.run("nonexistent", &[]);
    assert!(result.is_err());
}

#[test]
fn test_cli_plugin_duplicate_command_fails() {
    let mut plugin = CliPlugin::new("dup-cli");
    plugin
        .register(CliCommand::new("cmd", "first"), echo_handler)
        .unwrap();
    let result = plugin.register(CliCommand::new("cmd", "second"), echo_handler);
    assert!(result.is_err());
}

#[test]
fn test_cli_plugin_command_error_propagated() {
    let mut plugin = CliPlugin::new("err-cli");
    plugin
        .register(CliCommand::new("fail", "always fails"), fail_handler)
        .unwrap();
    let result = plugin.run("fail", &[]);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("intentional failure"));
}

#[test]
fn test_cli_plugin_list_commands() {
    let mut plugin = CliPlugin::new("list-cli");
    plugin
        .register(CliCommand::new("z-cmd", "last"), echo_handler)
        .unwrap();
    plugin
        .register(CliCommand::new("a-cmd", "first"), echo_handler)
        .unwrap();
    let cmds = plugin.list_commands();
    assert_eq!(cmds[0].name, "a-cmd");
    assert_eq!(cmds[1].name, "z-cmd");
}
