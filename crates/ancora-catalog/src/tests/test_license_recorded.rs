use crate::connector_entry::{ConnectorEntry, McpConfig};
use crate::install::{InstalledKind, ProjectRegistry};
use crate::metadata::{Author, License, Metadata, Version};
use crate::provider_entry::{ProviderBackend, ProviderEntry};
use crate::tool_entry::{ToolEffect, ToolEntry};
use crate::vectorstore_entry::{VectorStoreBackend, VectorStoreEntry};

fn meta_with_license(license: License) -> Metadata {
    Metadata::new(Version::new(1, 0, 0), Author::new("YASSERRMD"), license)
}

#[test]
fn installed_tool_records_apache2_license() {
    let mut reg = ProjectRegistry::new();
    let tool = ToolEntry::new(
        "lic-tool",
        "Licensed Tool",
        "A tool.",
        ToolEffect::None,
        meta_with_license(License::apache2()),
    );
    reg.install_tool(&tool).unwrap();
    let entry = reg.find_installed("lic-tool").unwrap();
    assert_eq!(entry.license.as_str(), "Apache-2.0");
}

#[test]
fn installed_tool_records_mit_license() {
    let mut reg = ProjectRegistry::new();
    let tool = ToolEntry::new(
        "mit-tool",
        "MIT Tool",
        "A tool.",
        ToolEffect::None,
        meta_with_license(License::mit()),
    );
    reg.install_tool(&tool).unwrap();
    let entry = reg.find_installed("mit-tool").unwrap();
    assert_eq!(entry.license.as_str(), "MIT");
}

#[test]
fn installed_connector_records_license() {
    let mut reg = ProjectRegistry::new();
    let conn = ConnectorEntry::new(
        "licensed-conn",
        "Licensed Connector",
        "A connector.",
        McpConfig::stdio("server", vec![]),
        meta_with_license(License::new("ISC")),
    );
    reg.install_connector(&conn).unwrap();
    let entry = reg.find_installed("licensed-conn").unwrap();
    assert_eq!(entry.license.as_str(), "ISC");
    assert_eq!(entry.kind, InstalledKind::Connector);
}

#[test]
fn installed_provider_records_license() {
    let mut reg = ProjectRegistry::new();
    let prov = ProviderEntry::new(
        "prov-1",
        "Test Provider",
        "A provider.",
        ProviderBackend::Anthropic,
        meta_with_license(License::apache2()),
    );
    reg.install_provider(&prov).unwrap();
    let entry = reg.find_installed("prov-1").unwrap();
    assert_eq!(entry.license.as_str(), "Apache-2.0");
}

#[test]
fn installed_vector_store_records_license() {
    let mut reg = ProjectRegistry::new();
    let vs = VectorStoreEntry::new(
        "vs-1",
        "My Vector Store",
        "A vector store.",
        VectorStoreBackend::InMemory,
        meta_with_license(License::mit()),
    );
    reg.install_vector_store(&vs).unwrap();
    let entry = reg.find_installed("vs-1").unwrap();
    assert_eq!(entry.license.as_str(), "MIT");
}
