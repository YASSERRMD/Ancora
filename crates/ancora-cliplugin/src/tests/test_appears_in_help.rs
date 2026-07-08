use crate::help::{build_plugin_help_section, compose_help, HelpConfig};
use crate::interface::CommandSpec;

#[test]
fn test_command_appears_in_help_section() {
    let specs = vec![
        CommandSpec::new("do-thing", "Does the thing", "Long description of do-thing"),
        CommandSpec::new("other-cmd", "Does other", "Long description of other-cmd"),
    ];

    let config = HelpConfig::default();
    let section = build_plugin_help_section(&specs, "Plugin Commands", &config);
    let rendered = section.render();

    assert!(
        rendered.contains("Plugin Commands"),
        "section title should appear"
    );
    assert!(rendered.contains("do-thing"), "command name should appear");
    assert!(
        rendered.contains("Does the thing"),
        "short help should appear"
    );
    assert!(
        rendered.contains("other-cmd"),
        "second command should appear"
    );
}

#[test]
fn test_aliases_appear_in_help_when_enabled() {
    let spec = CommandSpec::new("greet", "Greet someone", "Long").with_alias("hello");
    let config = HelpConfig {
        show_aliases: true,
        ..Default::default()
    };

    let section = build_plugin_help_section(&[spec], "Plugin Commands", &config);
    let rendered = section.render();

    assert!(
        rendered.contains("hello"),
        "alias should appear when show_aliases is true"
    );
}

#[test]
fn test_aliases_hidden_when_disabled() {
    let spec = CommandSpec::new("greet", "Greet someone", "Long").with_alias("hello");
    let config = HelpConfig {
        show_aliases: false,
        ..Default::default()
    };

    let section = build_plugin_help_section(&[spec], "Plugin Commands", &config);
    let rendered = section.render();

    assert!(
        !rendered.contains("hello"),
        "alias should be hidden when show_aliases is false"
    );
}

#[test]
fn test_compose_help_merges_builtin_and_plugin_sections() {
    let builtin = "Usage: ancora [OPTIONS] COMMAND\n\nOptions:\n  --help  Show help\n";
    let spec = CommandSpec::new("plugin-cmd", "A plugin command", "Long");
    let config = HelpConfig::default();
    let section = build_plugin_help_section(&[spec], "Plugin Commands", &config);

    let composed = compose_help(builtin, &[section]);

    assert!(
        composed.contains("Usage: ancora"),
        "builtin help should be present"
    );
    assert!(
        composed.contains("Plugin Commands"),
        "plugin section should be present"
    );
    assert!(
        composed.contains("plugin-cmd"),
        "plugin command should appear in composed help"
    );
}
