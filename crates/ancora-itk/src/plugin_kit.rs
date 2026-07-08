/// Conformance kit for plugin extensions.
use std::collections::HashMap;

/// Lifecycle events a plugin must handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleEvent {
    Init,
    Shutdown,
}

/// Trait that every plugin extension must satisfy.
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_event(&mut self, event: LifecycleEvent) -> Result<(), String>;
    fn metadata(&self) -> HashMap<String, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against a [`Plugin`].
pub struct PluginKit;

impl PluginKit {
    pub fn new() -> Self {
        PluginKit
    }

    pub fn run<P: Plugin>(&self, plugin: &mut P) -> Vec<CheckResult> {
        vec![
            self.check_name(plugin),
            self.check_version(plugin),
            self.check_init(plugin),
            self.check_shutdown(plugin),
            self.check_metadata(plugin),
        ]
    }

    fn check_name<P: Plugin>(&self, plugin: &P) -> CheckResult {
        if plugin.name().is_empty() {
            CheckResult {
                name: "plugin_name_nonempty".into(),
                passed: false,
                message: "Plugin name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "plugin_name_nonempty".into(),
                passed: true,
                message: format!("Plugin name: {}", plugin.name()),
            }
        }
    }

    fn check_version<P: Plugin>(&self, plugin: &P) -> CheckResult {
        let v = plugin.version();
        if v.is_empty() {
            CheckResult {
                name: "plugin_version_nonempty".into(),
                passed: false,
                message: "Plugin version must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "plugin_version_nonempty".into(),
                passed: true,
                message: format!("Version: {v}"),
            }
        }
    }

    fn check_init<P: Plugin>(&self, plugin: &mut P) -> CheckResult {
        match plugin.on_event(LifecycleEvent::Init) {
            Ok(()) => CheckResult {
                name: "plugin_handles_init".into(),
                passed: true,
                message: "Init event handled".into(),
            },
            Err(e) => CheckResult {
                name: "plugin_handles_init".into(),
                passed: false,
                message: format!("Init event failed: {e}"),
            },
        }
    }

    fn check_shutdown<P: Plugin>(&self, plugin: &mut P) -> CheckResult {
        match plugin.on_event(LifecycleEvent::Shutdown) {
            Ok(()) => CheckResult {
                name: "plugin_handles_shutdown".into(),
                passed: true,
                message: "Shutdown event handled".into(),
            },
            Err(e) => CheckResult {
                name: "plugin_handles_shutdown".into(),
                passed: false,
                message: format!("Shutdown event failed: {e}"),
            },
        }
    }

    fn check_metadata<P: Plugin>(&self, plugin: &P) -> CheckResult {
        let meta = plugin.metadata();
        CheckResult {
            name: "plugin_metadata_returns".into(),
            passed: true,
            message: format!("{} metadata key(s)", meta.len()),
        }
    }
}

impl Default for PluginKit {
    fn default() -> Self {
        Self::new()
    }
}
