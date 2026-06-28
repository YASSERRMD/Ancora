use crate::component::{Component, ComponentKind, License};
use crate::sbom::{Sbom, SbomFormat};
use crate::stats::SbomStats;
fn make_component(id: &str, kind: ComponentKind, license: License) -> Component {
    Component::new(id, "comp", "1.0", kind, license, "vendor", "sha")
}
#[test]
fn stats_total_components() {
    let mut sbom = Sbom::new("s1", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", ComponentKind::Library, License::Mit));
    sbom.add_component(make_component("c2", ComponentKind::Binary, License::Apache2));
    let stats = SbomStats::from(&sbom);
    assert_eq!(stats.total_components, 2);
}
#[test]
fn stats_open_source_count() {
    let mut sbom = Sbom::new("s2", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", ComponentKind::Library, License::Mit));
    sbom.add_component(make_component("c2", ComponentKind::Library, License::Proprietary));
    let stats = SbomStats::from(&sbom);
    assert_eq!(stats.open_source_count, 1);
}
#[test]
fn stats_by_kind_uses_string_keys() {
    let mut sbom = Sbom::new("s3", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", ComponentKind::Library, License::Mit));
    sbom.add_component(make_component("c2", ComponentKind::Library, License::Apache2));
    let stats = SbomStats::from(&sbom);
    assert_eq!(*stats.by_kind.get("LIBRARY").unwrap_or(&0), 2);
}
#[test]
fn stats_by_license_uses_string_keys() {
    let mut sbom = Sbom::new("s4", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", ComponentKind::Library, License::Mit));
    sbom.add_component(make_component("c2", ComponentKind::Library, License::Mit));
    let stats = SbomStats::from(&sbom);
    assert_eq!(*stats.by_license.get("MIT").unwrap_or(&0), 2);
}
