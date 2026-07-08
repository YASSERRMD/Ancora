#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::sbom::{Sbom, SbomFormat};
    use crate::stats::SbomStats;

    fn make_component(id: &str, license: License) -> Component {
        Component::new(
            id,
            "comp",
            "1.0.0",
            ComponentKind::Library,
            license,
            "vendor",
            "sha256:00",
        )
    }

    #[test]
    fn test_oss_rate_all_open_source() {
        let mut sbom = Sbom::new("sbom-oss1", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1", License::Mit));
        sbom.add_component(make_component("c2", License::Apache2));

        let stats = SbomStats::from(&sbom);
        assert!((stats.oss_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_oss_rate_none_open_source() {
        let mut sbom = Sbom::new("sbom-oss2", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1", License::Proprietary));
        sbom.add_component(make_component("c2", License::Unknown));

        let stats = SbomStats::from(&sbom);
        assert!((stats.oss_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_oss_rate_half_open_source() {
        let mut sbom = Sbom::new("sbom-oss3", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1", License::Mit));
        sbom.add_component(make_component("c2", License::Proprietary));

        let stats = SbomStats::from(&sbom);
        assert!((stats.oss_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_oss_rate_empty_sbom() {
        let sbom = Sbom::new("sbom-empty", "t1", SbomFormat::CycloneDx, 1000);
        let stats = SbomStats::from(&sbom);
        assert!((stats.oss_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_oss_rate_three_quarters() {
        let mut sbom = Sbom::new("sbom-oss4", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1", License::Mit));
        sbom.add_component(make_component("c2", License::Apache2));
        sbom.add_component(make_component("c3", License::Gpl3));
        sbom.add_component(make_component("c4", License::Proprietary));

        let stats = SbomStats::from(&sbom);
        assert!((stats.oss_rate() - 0.75).abs() < f64::EPSILON);
    }
}
