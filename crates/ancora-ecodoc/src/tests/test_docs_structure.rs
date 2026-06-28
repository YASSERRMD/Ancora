//! Tests verifying that the docs module structure is consistent.

use crate::{
    catalog_format, cli_plugins, contrib_templates, examples_index,
    extensibility_overview, fw_adapters, governance, graph_builder,
    interop_kit, market_trust, packaging, plugin_safety, plugin_sdk,
    quickstart, readiness, recipes, registry, sdk_extensions, security,
    troubleshooting,
};

#[test]
fn extensibility_overview_has_extension_points() {
    assert!(!extensibility_overview::all_extension_points().is_empty());
}

#[test]
fn plugin_sdk_version_is_set() {
    assert!(!plugin_sdk::SDK_VERSION.is_empty());
}

#[test]
fn catalog_schema_version_is_positive() {
    assert!(catalog_format::CATALOG_SCHEMA_VERSION > 0);
}

#[test]
fn cli_registry_starts_empty() {
    let reg = cli_plugins::CliRegistry::new();
    assert!(reg.list().is_empty());
}

#[test]
fn graph_new_is_empty() {
    let g = graph_builder::TaskGraph::new();
    assert_eq!(g.node_count(), 0);
}

#[test]
fn contrib_has_mandatory_templates() {
    assert!(!contrib_templates::mandatory_templates().is_empty());
}

#[test]
fn interop_suite_passes_trivially() {
    let suite = interop_kit::InteropSuite::new();
    assert!(suite.all_pass());
}

#[test]
fn market_trust_score_bounded() {
    use market_trust::{AuditStatus, TrustSignals};
    let signals = TrustSignals {
        download_count: 1000,
        review_score: 3.5,
        review_count: 10,
        audit_status: AuditStatus::NotAudited,
        maintained: false,
    };
    let score = signals.score();
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn sdk_extensions_builder_round_trips() {
    let meta = sdk_extensions::PluginMetaBuilder::new()
        .name("test")
        .version("0.1.0")
        .author("Tester")
        .build()
        .unwrap();
    assert_eq!(meta.name, "test");
}

#[test]
fn recipes_index_non_empty() {
    assert!(!recipes::builtin_recipes().is_empty());
}

#[test]
fn fw_adapters_stub_connects() {
    use fw_adapters::{AdapterConfig, FrameworkAdapter, FrameworkKind, StubAdapter};
    let adapter = StubAdapter::new(FrameworkKind::Airflow);
    let config = AdapterConfig {
        framework: FrameworkKind::Airflow,
        endpoint: None,
        namespace: None,
    };
    adapter.connect(&config).unwrap();
    assert!(adapter.is_connected());
}

#[test]
fn governance_semver_display() {
    let v = governance::SemVer::new(1, 2, 3);
    assert_eq!(v.to_string(), "1.2.3");
}

#[test]
fn packaging_checklist_non_empty() {
    assert!(!packaging::release_checklist().is_empty());
}

#[test]
fn examples_index_non_empty() {
    assert!(!examples_index::example_index().is_empty());
}

#[test]
fn security_requirements_non_empty() {
    assert!(!security::security_requirements().is_empty());
}

#[test]
fn troubleshooting_db_non_empty() {
    assert!(!troubleshooting::known_issues().is_empty());
}

#[test]
fn readiness_criteria_non_empty() {
    assert!(!readiness::readiness_criteria().is_empty());
}

#[test]
fn quickstart_steps_non_empty() {
    assert!(!quickstart::quickstart_steps().is_empty());
}

#[test]
fn registry_starts_empty() {
    let reg = registry::Registry::new();
    assert!(reg.is_empty());
}

#[test]
fn plugin_safety_untrusted_has_no_caps() {
    use plugin_safety::{allowed_capabilities, TrustLevel};
    assert!(allowed_capabilities(TrustLevel::Untrusted).is_empty());
}
