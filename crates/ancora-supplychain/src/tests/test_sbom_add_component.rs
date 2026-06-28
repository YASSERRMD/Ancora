#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::sbom::{Sbom, SbomFormat};

    fn make_sbom() -> Sbom {
        Sbom::new("sbom-1", "tenant-1", SbomFormat::CycloneDx, 100)
    }

    fn make_component(id: &str, name: &str) -> Component {
        Component::new(id, name, "1.0.0", ComponentKind::Library, License::Mit, "acme", "sha256:00")
    }

    #[test]
    fn test_empty_sbom_has_zero_components() {
        let sbom = make_sbom();
        assert_eq!(sbom.component_count(), 0);
    }

    #[test]
    fn test_add_one_component_increments_count() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("c1", "lib-a"));
        assert_eq!(sbom.component_count(), 1);
    }

    #[test]
    fn test_add_multiple_components_increments_count() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("c1", "lib-a"));
        sbom.add_component(make_component("c2", "lib-b"));
        sbom.add_component(make_component("c3", "lib-c"));
        assert_eq!(sbom.component_count(), 3);
    }

    #[test]
    fn test_sbom_stores_tenant_id() {
        let sbom = make_sbom();
        assert_eq!(sbom.tenant_id, "tenant-1");
    }

    #[test]
    fn test_sbom_stores_format() {
        let sbom = make_sbom();
        assert!(matches!(sbom.format, SbomFormat::CycloneDx));
    }
}
