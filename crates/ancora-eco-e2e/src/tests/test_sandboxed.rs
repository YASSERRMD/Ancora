use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

/// Simulates a sandboxed plugin environment: each plugin gets its own isolated context.
struct SandboxedEnv {
    pub env_id: u64,
    pub plugins: Vec<Plugin>,
}

impl SandboxedEnv {
    fn new(env_id: u64) -> Self {
        SandboxedEnv {
            env_id,
            plugins: Vec::new(),
        }
    }

    fn spawn(&mut self, name: &str, version: &str) -> Result<(), String> {
        let template = PluginTemplate::new(name, version, "sandboxed", "main.rs");
        let id = self.env_id * 1000 + self.plugins.len() as u64;
        let mut plugin = Plugin::from_template(template, id)
            .ok_or_else(|| "template invalid".to_string())?;
        plugin.compile()?;
        plugin.install()?;
        plugin.start()?;
        self.plugins.push(plugin);
        Ok(())
    }

    fn running_count(&self) -> usize {
        self.plugins
            .iter()
            .filter(|p| p.state == PluginState::Running)
            .count()
    }

    fn terminate_all(&mut self) {
        for p in &mut self.plugins {
            if p.state == PluginState::Running {
                let _ = p.stop();
            }
        }
    }
}

#[test]
fn test_plugin_runs_sandboxed() {
    let mut env = SandboxedEnv::new(1);
    env.spawn("sandbox-plugin", "0.1.0").expect("spawn must succeed");
    assert_eq!(env.running_count(), 1);
}

#[test]
fn test_sandboxes_are_independent() {
    let mut env_a = SandboxedEnv::new(1);
    let mut env_b = SandboxedEnv::new(2);
    env_a.spawn("plugin-a", "1.0.0").unwrap();
    env_b.spawn("plugin-b", "1.0.0").unwrap();
    assert_eq!(env_a.running_count(), 1);
    assert_eq!(env_b.running_count(), 1);
    env_a.terminate_all();
    assert_eq!(env_a.running_count(), 0);
    assert_eq!(env_b.running_count(), 1);
}

#[test]
fn test_sandbox_residency_enforced() {
    // Each env has a unique env_id; plugin IDs include env_id as prefix.
    let mut env = SandboxedEnv::new(42);
    env.spawn("res-plugin", "1.0.0").unwrap();
    let plugin = &env.plugins[0];
    // Plugin id encodes env residency.
    assert_eq!(plugin.id / 1000, 42);
}
