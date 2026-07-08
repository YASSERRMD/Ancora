use crate::component::{Component, ComponentKind, License};
use crate::provenance::ProvenanceStore;
use crate::sbom::{Sbom, SbomFormat};
use crate::signature::SignatureStore;
use crate::validator::{SbomIssue, SbomValidator};
fn make_component(id: &str) -> Component {
    Component::new(
        id,
        "lib",
        "1.0",
        ComponentKind::Library,
        License::Mit,
        "vendor",
        "sha1",
    )
}
#[test]
fn empty_sbom_produces_empty_sbom_issue() {
    let sbom = Sbom::new("s1", "t1", SbomFormat::CycloneDx, 0);
    let sigs = SignatureStore::new();
    let prov = ProvenanceStore::new();
    let issues = SbomValidator::validate(&sbom, &sigs, &prov, false, false);
    assert!(issues.contains(&SbomIssue::EmptySbom));
}
#[test]
fn unsigned_component_reported_when_required() {
    let mut sbom = Sbom::new("s2", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1"));
    let sigs = SignatureStore::new();
    let prov = ProvenanceStore::new();
    let issues = SbomValidator::validate(&sbom, &sigs, &prov, true, false);
    assert!(issues.contains(&SbomIssue::UnsignedComponent("c1".to_string())));
}
#[test]
fn valid_sbom_with_sigs_and_prov_has_no_issues() {
    use crate::provenance::{ProvenanceKind, ProvenanceRecord};
    use crate::signature::{ComponentSignature, SignatureAlgorithm};
    let mut sbom = Sbom::new("s3", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1"));
    let mut sigs = SignatureStore::new();
    sigs.register(ComponentSignature::new(
        "c1",
        SignatureAlgorithm::Ed25519,
        "bot",
        "sig",
        0,
    ));
    let mut prov = ProvenanceStore::new();
    prov.record(ProvenanceRecord::new(
        "c1",
        ProvenanceKind::BuildSystem,
        "ci",
        "b1",
        0,
    ));
    assert!(SbomValidator::is_valid(&sbom, &sigs, &prov, true, true));
}
