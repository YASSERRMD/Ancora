use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

fn make_interop_plugin(id: u64) -> Plugin {
    let template = PluginTemplate::new(
        "interop-plugin",
        "0.2.0",
        "Interop kit test plugin",
        "src/interop.rs",
    );
    let mut plugin = Plugin::from_template(template, id).unwrap();
    plugin.compile().unwrap();
    plugin.install().unwrap();
    plugin
}

#[test]
fn test_plugin_passes_interop_state_check() {
    let plugin = make_interop_plugin(10);
    assert_eq!(plugin.state, PluginState::Installed);
}

#[test]
fn test_plugin_interop_start_stop() {
    let mut plugin = make_interop_plugin(11);
    plugin.start().expect("start must succeed");
    assert_eq!(plugin.state, PluginState::Running);
    plugin.stop().expect("stop must succeed");
    assert_eq!(plugin.state, PluginState::Stopped);
}

#[test]
fn test_double_start_fails() {
    let mut plugin = make_interop_plugin(12);
    plugin.start().unwrap();
    let result = plugin.start();
    assert!(result.is_err(), "double start must fail");
}
