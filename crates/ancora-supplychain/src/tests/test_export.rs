use crate::component::{Component, ComponentKind, License};
use crate::export::{sbom_to_csv, sbom_to_summary};
use crate::sbom::{Sbom, SbomFormat};
#[test]
fn sbom_to_csv_includes_header_and_component() {
    let mut sbom = Sbom::new("s1", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(Component::new(
        "c1",
        "openssl",
        "3.0",
        ComponentKind::Library,
        License::Apache2,
        "project",
        "sha",
    ));
    let csv = sbom_to_csv(&sbom);
    assert!(csv.starts_with("id,name,version"));
    assert!(csv.contains("openssl"));
}
#[test]
fn sbom_to_summary_contains_sbom_id_and_count() {
    let mut sbom = Sbom::new("s1", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(Component::new(
        "c1",
        "lib",
        "1.0",
        ComponentKind::Library,
        License::Mit,
        "vendor",
        "sha",
    ));
    let summary = sbom_to_summary(&sbom);
    assert!(summary.contains("s1"));
    assert!(summary.contains("1 components"));
}
