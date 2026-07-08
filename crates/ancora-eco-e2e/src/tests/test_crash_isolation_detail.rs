/// Additional crash isolation detail tests.
use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

fn running_plugin(id: u64) -> Plugin {
    let template = PluginTemplate::new(&format!("plugin-{}", id), "1.0.0", "test", "main.rs");
    let mut plugin = Plugin::from_template(template, id).unwrap();
    plugin.compile().unwrap();
    plugin.install().unwrap();
    plugin.start().unwrap();
    plugin
}

#[test]
fn test_crash_leaves_others_running() {
    let mut p1 = running_plugin(100);
    let mut p2 = running_plugin(101);
    let mut p3 = running_plugin(102);
    // Simulate crash of p2.
    p2.state = PluginState::Failed("oom".to_string());
    assert_eq!(p1.state, PluginState::Running);
    assert_eq!(p3.state, PluginState::Running);
    assert!(matches!(p2.state, PluginState::Failed(_)));
}

#[test]
fn test_failed_plugin_cannot_stop() {
    let mut plugin = running_plugin(200);
    plugin.state = PluginState::Failed("crashed".to_string());
    // Stop is only valid from Running; a failed plugin is not stoppable via normal API.
    let result = plugin.stop();
    assert!(result.is_err());
}
