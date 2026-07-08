#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::sbom::{Sbom, SbomFormat};

    fn make_component(id: &str, license: License) -> Component {
        Component::new(
            id,
            "comp",
            "1.0.0",
            ComponentKind::Library,
            license,
            "s",
            "sha256:00",
        )
    }

    fn make_sbom() -> Sbom {
        Sbom::new("sbom-prop", "tenant-3", SbomFormat::CycloneDx, 300)
    }

    #[test]
    fn test_empty_sbom_has_zero_proprietary() {
        let sbom = make_sbom();
        assert_eq!(sbom.proprietary_count(), 0);
    }

    #[test]
    fn test_proprietary_license_counted() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("p1", License::Proprietary));
        assert_eq!(sbom.proprietary_count(), 1);
    }

    #[test]
    fn test_unknown_license_counted_as_proprietary() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("u1", License::Unknown));
        assert_eq!(sbom.proprietary_count(), 1);
    }

    #[test]
    fn test_open_source_not_counted() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("m1", License::Mit));
        sbom.add_component(make_component("a1", License::Apache2));
        sbom.add_component(make_component("g1", License::Gpl3));
        assert_eq!(sbom.proprietary_count(), 0);
    }

    #[test]
    fn test_mixed_licenses_counted_correctly() {
        let mut sbom = make_sbom();
        sbom.add_component(make_component("m1", License::Mit));
        sbom.add_component(make_component("p1", License::Proprietary));
        sbom.add_component(make_component("u1", License::Unknown));
        sbom.add_component(make_component("a1", License::Apache2));
        assert_eq!(sbom.proprietary_count(), 2);
    }
}
