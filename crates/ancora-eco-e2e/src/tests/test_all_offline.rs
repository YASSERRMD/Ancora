use crate::airgap_e2e::{AirgapBundle, AirgapRegistry, BundledPlugin};
use crate::builder_e2e::{Node, PluginGraph};
use crate::catalog_e2e::{CatalogTool, ToolCatalog};
/// Verifies that all ecosystem operations work without any network calls.
/// All constructs are in-memory; no I/O, no sockets.
use crate::plugin_e2e::{Plugin, PluginTemplate};
use crate::recipe_e2e::{Recipe, RecipeRunner, RecipeStep};
use crate::registry_e2e::{LocalRegistry, RegistryEntry};
use crate::trust_e2e::{PluginManifest, TrustGate, TrustLevel, TrustPolicy};

#[test]
fn test_all_offline_plugin_roundtrip() {
    let template = PluginTemplate::new("offline-plugin", "1.0.0", "test", "main.rs");
    let mut plugin = Plugin::from_template(template, 1).unwrap();
    plugin.compile().unwrap();
    plugin.install().unwrap();
    plugin.start().unwrap();
    plugin.stop().unwrap();
    // All steps completed with zero network calls.
}

#[test]
fn test_all_offline_registry_roundtrip() {
    let mut reg = LocalRegistry::new();
    reg.publish(RegistryEntry::new("p", "1.0.0", "org"))
        .unwrap();
    assert!(reg.latest("p").is_some());
}

#[test]
fn test_all_offline_catalog_roundtrip() {
    let mut catalog = ToolCatalog::new();
    catalog.install(CatalogTool::new("t", "desc", 2)).unwrap();
    assert_eq!(catalog.count(), 1);
}

#[test]
fn test_all_offline_graph_roundtrip() {
    let mut graph = PluginGraph::new();
    graph.add_node(Node::new(1, "A", "p")).unwrap();
    graph.add_node(Node::new(2, "B", "q")).unwrap();
    graph.add_edge(1, 2).unwrap();
    let order = graph.topological_order().unwrap();
    assert_eq!(order, vec![1, 2]);
}

#[test]
fn test_all_offline_recipe_roundtrip() {
    let mut recipe = Recipe::new("offline-recipe", "1.0.0");
    recipe.add_step(RecipeStep::new("step1", "plugin", "cmd", vec![]));
    let mut runner = RecipeRunner::new();
    runner.install(recipe).unwrap();
    let results = runner.run("offline-recipe").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_all_offline_trust_roundtrip() {
    let gate = TrustGate::new(TrustPolicy::new(TrustLevel::Community));
    let manifest = PluginManifest::new("p", TrustLevel::Verified, true, Some("cs"));
    gate.check(&manifest).unwrap();
}

#[test]
fn test_all_offline_airgap_roundtrip() {
    let mut bundle = AirgapBundle::new();
    bundle
        .add(BundledPlugin::new("p", "1.0.0", b"data".to_vec()))
        .unwrap();
    let mut reg = AirgapRegistry::new(bundle);
    reg.install("p", "1.0.0").unwrap();
    assert!(reg.is_installed("p", "1.0.0"));
}
