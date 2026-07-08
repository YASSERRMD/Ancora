use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

#[test]
fn test_author_plugin_from_template() {
    let template = PluginTemplate::new("my-plugin", "0.1.0", "A test plugin", "src/lib.rs");
    assert!(template.is_valid());
    let plugin = Plugin::from_template(template.clone(), 1).expect("plugin must be created");
    assert_eq!(plugin.template.name, "my-plugin");
    assert_eq!(plugin.state, PluginState::Created);
}

#[test]
fn test_invalid_template_rejected() {
    let template = PluginTemplate::new("", "0.1.0", "desc", "main.rs");
    assert!(!template.is_valid());
    let result = Plugin::from_template(template, 2);
    assert!(result.is_none());
}

#[test]
fn test_plugin_full_lifecycle() {
    let template = PluginTemplate::new("lifecycle-plugin", "1.0.0", "desc", "main.rs");
    let mut plugin = Plugin::from_template(template, 3).unwrap();
    plugin.compile().expect("compile must succeed");
    assert_eq!(plugin.state, PluginState::Compiled);
    plugin.install().expect("install must succeed");
    assert_eq!(plugin.state, PluginState::Installed);
    plugin.start().expect("start must succeed");
    assert_eq!(plugin.state, PluginState::Running);
    plugin.stop().expect("stop must succeed");
    assert_eq!(plugin.state, PluginState::Stopped);
}
