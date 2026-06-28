use crate::registry_e2e::{LocalRegistry, RegistryEntry};
use crate::plugin_e2e::{Plugin, PluginState, PluginTemplate};

fn populate_registry() -> LocalRegistry {
    let mut reg = LocalRegistry::new();
    reg.publish(RegistryEntry::new("tool-alpha", "1.0.0", "trusted-org")).unwrap();
    reg.publish(RegistryEntry::new("tool-beta", "2.0.0", "trusted-org")).unwrap();
    reg
}

fn install_from_registry(registry: &LocalRegistry, name: &str, version: &str) -> Option<Plugin> {
    let entry = registry.all_versions(name).into_iter().find(|e| e.version == version)?;
    let template = PluginTemplate::new(&entry.name, &entry.version, "from registry", "main.rs");
    let mut plugin = Plugin::from_template(template, 100)?;
    plugin.compile().ok()?;
    plugin.install().ok()?;
    Some(plugin)
}

#[test]
fn test_install_plugin_from_registry() {
    let registry = populate_registry();
    let plugin = install_from_registry(&registry, "tool-alpha", "1.0.0")
        .expect("must install from registry");
    assert_eq!(plugin.state, PluginState::Installed);
    assert_eq!(plugin.template.name, "tool-alpha");
}

#[test]
fn test_install_missing_plugin_returns_none() {
    let registry = populate_registry();
    let result = install_from_registry(&registry, "nonexistent", "1.0.0");
    assert!(result.is_none());
}

#[test]
fn test_install_then_run() {
    let registry = populate_registry();
    let mut plugin = install_from_registry(&registry, "tool-beta", "2.0.0").unwrap();
    plugin.start().expect("start must succeed");
    assert_eq!(plugin.state, PluginState::Running);
}
