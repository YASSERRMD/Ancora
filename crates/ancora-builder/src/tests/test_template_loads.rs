/// test_template_loads - Test that preset templates load and produce valid specs.

use crate::templates::{TemplateCategory, TemplateRegistry};
use crate::validation::validate_spec;
use crate::runner::{run_spec, LocalBackendConfig, RunStatus};

#[test]
fn all_templates_load() {
    let registry = TemplateRegistry::default_registry();
    assert!(!registry.is_empty(), "template registry should not be empty");
    assert!(registry.len() >= 4, "expected at least 4 built-in templates");
}

#[test]
fn single_agent_template_loads() {
    let registry = TemplateRegistry::default_registry();
    let t = registry.get("single_agent").expect("single_agent template should exist");
    assert!(!t.spec.nodes.is_empty());
    assert!(!t.spec.edges.is_empty());
}

#[test]
fn rag_pipeline_template_loads() {
    let registry = TemplateRegistry::default_registry();
    let t = registry.get("rag_pipeline").expect("rag_pipeline template should exist");
    assert_eq!(t.category, TemplateCategory::RagPipeline);
    assert!(!t.spec.nodes.is_empty());
}

#[test]
fn all_templates_produce_valid_specs() {
    let registry = TemplateRegistry::default_registry();
    for template in registry.all() {
        let report = validate_spec(&template.spec);
        assert!(
            report.is_valid(),
            "template '{}' produced invalid spec: {:?}",
            template.id,
            report.diagnostics
        );
    }
}

#[test]
fn template_instantiation_renames_spec() {
    let registry = TemplateRegistry::default_registry();
    let spec = registry.instantiate("rag_pipeline", "my_custom_rag").unwrap();
    assert_eq!(spec.name, "my_custom_rag");
}

#[test]
fn instantiated_spec_is_valid() {
    let registry = TemplateRegistry::default_registry();
    let spec = registry.instantiate("agent_verifier", "my_agent_verifier").unwrap();
    let report = validate_spec(&spec);
    assert!(report.is_valid(), "{:?}", report.diagnostics);
}

#[test]
fn instantiated_spec_can_be_run() {
    let registry = TemplateRegistry::default_registry();
    let spec = registry.instantiate("single_agent", "run_template_test").unwrap();
    let config = LocalBackendConfig { offline: true, ..Default::default() };
    let result = run_spec(&spec, &config).expect("template run should succeed");
    assert_eq!(result.status, RunStatus::Completed);
}

#[test]
fn template_search_works() {
    let registry = TemplateRegistry::default_registry();
    let results = registry.search("rag");
    assert!(!results.is_empty(), "search for 'rag' should return results");
}

#[test]
fn multi_agent_template_has_multiple_agents() {
    let registry = TemplateRegistry::default_registry();
    let t = registry.get("multi_agent").expect("multi_agent template should exist");
    let agent_count = t.spec.nodes.iter().filter(|n| n.kind.starts_with("agent.")).count();
    assert!(agent_count >= 2, "multi_agent template should have at least 2 agents");
}
