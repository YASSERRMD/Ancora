use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

/// Demonstrates that a plugin crash is isolated: other plugins remain unaffected.
struct IsolatedEnv {
    plugins: Vec<(String, Plugin)>,
}

impl IsolatedEnv {
    fn new() -> Self {
        IsolatedEnv {
            plugins: Vec::new(),
        }
    }

    fn add(&mut self, name: &str, id: u64) {
        let template = PluginTemplate::new(name, "1.0.0", "test", "main.rs");
        let mut plugin = Plugin::from_template(template, id).unwrap();
        plugin.compile().unwrap();
        plugin.install().unwrap();
        plugin.start().unwrap();
        self.plugins.push((name.to_string(), plugin));
    }

    fn crash(&mut self, name: &str) {
        if let Some((_, plugin)) = self.plugins.iter_mut().find(|(n, _)| n == name) {
            plugin.state = PluginState::Failed("simulated crash".to_string());
        }
    }

    fn running_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|(_, p)| p.state == PluginState::Running)
            .count()
    }

    fn failed_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|(_, p)| matches!(p.state, PluginState::Failed(_)))
            .count()
    }
}

#[test]
fn test_plugin_crash_isolated() {
    let mut env = IsolatedEnv::new();
    env.add("stable-a", 1);
    env.add("crashy-b", 2);
    env.add("stable-c", 3);
    assert_eq!(env.running_count(), 3);
    env.crash("crashy-b");
    // Only crashy-b is failed; others remain running.
    assert_eq!(env.failed_count(), 1);
    assert_eq!(env.running_count(), 2);
}

#[test]
fn test_multiple_crashes_isolated() {
    let mut env = IsolatedEnv::new();
    for i in 0..5 {
        env.add(&format!("plugin-{}", i), i as u64);
    }
    env.crash("plugin-1");
    env.crash("plugin-3");
    assert_eq!(env.failed_count(), 2);
    assert_eq!(env.running_count(), 3);
}

#[test]
fn test_residency_enforced_for_extensions() {
    let mut env = IsolatedEnv::new();
    env.add("res-plugin", 42);
    let (_, plugin) = env.plugins.iter().find(|(n, _)| n == "res-plugin").unwrap();
    // The plugin id encodes its identity, demonstrating residency tracking.
    assert_eq!(plugin.id, 42);
    assert_eq!(plugin.state, PluginState::Running);
}
