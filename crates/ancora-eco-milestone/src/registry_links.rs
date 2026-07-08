/// Registry and catalog guide links for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct RegistryLink {
    pub label: &'static str,
    pub url: &'static str,
    pub description: &'static str,
}

impl RegistryLink {
    pub const fn new(label: &'static str, url: &'static str, description: &'static str) -> Self {
        Self {
            label,
            url,
            description,
        }
    }

    pub fn is_docs_link(&self) -> bool {
        self.url.contains("/docs/")
    }
}

pub fn registry_links() -> Vec<RegistryLink> {
    vec![
        RegistryLink::new(
            "Plugin Registry",
            "https://ancora.dev/registry",
            "Browse and search published plugins",
        ),
        RegistryLink::new(
            "Plugin Catalog Docs",
            "https://ancora.dev/docs/catalog",
            "Guide to discovering and installing plugins via the catalog",
        ),
        RegistryLink::new(
            "Publishing Guide",
            "https://ancora.dev/docs/registry/publish",
            "Step-by-step guide to publishing a plugin",
        ),
        RegistryLink::new(
            "Registry API Reference",
            "https://ancora.dev/docs/registry/api",
            "REST API reference for the registry service",
        ),
        RegistryLink::new(
            "Catalog API Reference",
            "https://ancora.dev/docs/catalog/api",
            "REST API reference for the catalog service",
        ),
    ]
}

pub fn link_count() -> usize {
    registry_links().len()
}
