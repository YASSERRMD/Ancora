#[cfg(test)]
mod tests {
    use crate::builder::ComponentBuilder;
    use crate::component::{ComponentKind, License};

    #[test]
    fn test_builder_defaults_kind_is_library() {
        let comp = ComponentBuilder::new("id-1", "mylib", "1.0.0").build();
        assert!(matches!(comp.kind, ComponentKind::Library));
    }

    #[test]
    fn test_builder_defaults_license_is_unknown() {
        let comp = ComponentBuilder::new("id-2", "mylib", "1.0.0").build();
        assert!(matches!(comp.license, License::Unknown));
    }

    #[test]
    fn test_builder_sets_id() {
        let comp = ComponentBuilder::new("comp-abc", "lib", "2.0.0").build();
        assert_eq!(comp.id, "comp-abc");
    }

    #[test]
    fn test_builder_sets_name() {
        let comp = ComponentBuilder::new("id", "my-special-lib", "0.1.0").build();
        assert_eq!(comp.name, "my-special-lib");
    }

    #[test]
    fn test_builder_sets_version() {
        let comp = ComponentBuilder::new("id", "lib", "3.2.1").build();
        assert_eq!(comp.version, "3.2.1");
    }

    #[test]
    fn test_builder_set_kind() {
        let comp = ComponentBuilder::new("id", "lib", "1.0.0")
            .kind(ComponentKind::Container)
            .build();
        assert!(matches!(comp.kind, ComponentKind::Container));
    }

    #[test]
    fn test_builder_set_license() {
        let comp = ComponentBuilder::new("id", "lib", "1.0.0")
            .license(License::Apache2)
            .build();
        assert!(matches!(comp.license, License::Apache2));
    }

    #[test]
    fn test_builder_set_supplier() {
        let comp = ComponentBuilder::new("id", "lib", "1.0.0")
            .supplier("acme-corp")
            .build();
        assert_eq!(comp.supplier, "acme-corp");
    }

    #[test]
    fn test_builder_set_digest() {
        let comp = ComponentBuilder::new("id", "lib", "1.0.0")
            .digest("sha256:deadbeef")
            .build();
        assert_eq!(comp.digest, "sha256:deadbeef");
    }

    #[test]
    fn test_builder_full_chain() {
        let comp = ComponentBuilder::new("full-id", "full-lib", "9.9.9")
            .kind(ComponentKind::Framework)
            .license(License::Bsd3)
            .supplier("bigcorp")
            .digest("sha256:cafebabe")
            .build();

        assert_eq!(comp.id, "full-id");
        assert_eq!(comp.name, "full-lib");
        assert_eq!(comp.version, "9.9.9");
        assert!(matches!(comp.kind, ComponentKind::Framework));
        assert!(matches!(comp.license, License::Bsd3));
        assert_eq!(comp.supplier, "bigcorp");
        assert_eq!(comp.digest, "sha256:cafebabe");
    }
}
