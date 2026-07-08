use crate::component::{Component, ComponentKind, License};
use crate::policy::SupplyChainPolicy;
use crate::provenance::{ProvenanceKind, ProvenanceRecord, ProvenanceStore};
use crate::report::SupplyChainReport;
use crate::sbom::{Sbom, SbomFormat};
use crate::signature::{ComponentSignature, SignatureAlgorithm, SignatureStore};
fn make_component(id: &str, license: License) -> Component {
    Component::new(
        id,
        "lib",
        "1.0",
        ComponentKind::Library,
        license,
        "vendor",
        "sha",
    )
}
fn make_sig(component_id: &str) -> ComponentSignature {
    ComponentSignature::new(
        component_id,
        SignatureAlgorithm::Ed25519,
        "signer",
        "sig",
        0,
    )
}
#[test]
fn report_signed_count() {
    let mut sbom = Sbom::new("s1", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", License::Mit));
    sbom.add_component(make_component("c2", License::Apache2));
    let mut sigs = SignatureStore::new();
    sigs.register(make_sig("c1"));
    let prov = ProvenanceStore::new();
    let policy = SupplyChainPolicy::new("t1");
    let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1);
    assert_eq!(report.signed_count, 1);
    assert_eq!(report.unsigned_count, 1);
}
#[test]
fn report_denied_count_with_deny_policy() {
    let mut sbom = Sbom::new("s2", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", License::Mit));
    sbom.add_component(make_component("c2", License::Gpl3));
    let sigs = SignatureStore::new();
    let prov = ProvenanceStore::new();
    let policy = SupplyChainPolicy::new("t1").deny_license(License::Gpl3);
    let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1);
    assert_eq!(report.denied_count, 1);
    assert!(!report.is_compliant());
}
#[test]
fn report_provenance_count() {
    let mut sbom = Sbom::new("s3", "t1", SbomFormat::CycloneDx, 0);
    sbom.add_component(make_component("c1", License::Mit));
    sbom.add_component(make_component("c2", License::Apache2));
    let sigs = SignatureStore::new();
    let mut prov = ProvenanceStore::new();
    prov.record(ProvenanceRecord::new(
        "c1",
        ProvenanceKind::BuildSystem,
        "ci",
        "b1",
        0,
    ));
    let policy = SupplyChainPolicy::new("t1");
    let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1);
    assert_eq!(report.provenance_count, 1);
}
