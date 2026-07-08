#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::sbom::{Sbom, SbomFormat};

    fn make_sbom_with_components() -> Sbom {
        let mut sbom = Sbom::new("sbom-find", "tenant-2", SbomFormat::Spdx, 200);
        sbom.add_component(Component::new(
            "id-alpha",
            "alpha-lib",
            "1.0.0",
            ComponentKind::Library,
            License::Mit,
            "vendor-a",
            "sha256:aa",
        ));
        sbom.add_component(Component::new(
            "id-beta",
            "beta-lib",
            "2.0.0",
            ComponentKind::Framework,
            License::Apache2,
            "vendor-b",
            "sha256:bb",
        ));
        sbom
    }

    #[test]
    fn test_find_by_name_returns_matching_component() {
        let sbom = make_sbom_with_components();
        let result = sbom.find_by_name("alpha-lib");
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "id-alpha");
    }

    #[test]
    fn test_find_by_name_returns_none_for_missing() {
        let sbom = make_sbom_with_components();
        assert!(sbom.find_by_name("nonexistent-lib").is_none());
    }

    #[test]
    fn test_find_by_id_returns_matching_component() {
        let sbom = make_sbom_with_components();
        let result = sbom.find_by_id("id-beta");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "beta-lib");
    }

    #[test]
    fn test_find_by_id_returns_none_for_missing() {
        let sbom = make_sbom_with_components();
        assert!(sbom.find_by_id("id-missing").is_none());
    }

    #[test]
    fn test_find_by_name_empty_sbom_returns_none() {
        let sbom = Sbom::new("empty", "tenant-x", SbomFormat::Spdx, 1);
        assert!(sbom.find_by_name("anything").is_none());
    }
}
