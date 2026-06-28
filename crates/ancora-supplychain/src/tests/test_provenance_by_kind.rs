use crate::provenance::{ProvenanceKind, ProvenanceRecord, ProvenanceStore};
fn make_record(component_id: &str, kind: ProvenanceKind) -> ProvenanceRecord {
    ProvenanceRecord::new(component_id, kind, "source", "build-id", 0)
}
#[test]
fn by_kind_build_system_filters_correctly() {
    let mut store = ProvenanceStore::new();
    store.record(make_record("c1", ProvenanceKind::BuildSystem));
    store.record(make_record("c2", ProvenanceKind::Vcs));
    store.record(make_record("c3", ProvenanceKind::BuildSystem));
    assert_eq!(store.by_kind(&ProvenanceKind::BuildSystem).len(), 2);
}
#[test]
fn by_kind_vcs_filters_correctly() {
    let mut store = ProvenanceStore::new();
    store.record(make_record("c1", ProvenanceKind::Vcs));
    store.record(make_record("c2", ProvenanceKind::Registry));
    let results = store.by_kind(&ProvenanceKind::Vcs);
    assert_eq!(results.len(), 1);
}
#[test]
fn by_kind_returns_empty_for_no_match() {
    let mut store = ProvenanceStore::new();
    store.record(make_record("c1", ProvenanceKind::BuildSystem));
    assert!(store.by_kind(&ProvenanceKind::ArtifactStore).is_empty());
}
#[test]
fn provenance_kind_display() {
    assert_eq!(format!("{}", ProvenanceKind::BuildSystem), "BUILD_SYSTEM");
    assert_eq!(format!("{}", ProvenanceKind::Vcs), "VCS");
    assert_eq!(format!("{}", ProvenanceKind::Registry), "REGISTRY");
    assert_eq!(format!("{}", ProvenanceKind::ArtifactStore), "ARTIFACT_STORE");
}
