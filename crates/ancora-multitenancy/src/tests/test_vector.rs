#[cfg(test)]
mod tests {
    use crate::{TenantConfig, TenantContext, TenantRegistry};
    use crate::vector_scope::vector_collection;

    #[test]
    fn vector_collections_differ_across_tenants() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let b = reg.create("b", TenantConfig::default());
        assert_ne!(
            vector_collection(&TenantContext::new(a), "embeddings"),
            vector_collection(&TenantContext::new(b), "embeddings")
        );
    }

    #[test]
    fn vector_collection_includes_base_name() {
        let mut reg = TenantRegistry::new();
        let a = reg.create("a", TenantConfig::default());
        let name = vector_collection(&TenantContext::new(a), "docs");
        assert!(name.contains("docs"), "collection name should contain base name");
    }
}
