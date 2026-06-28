use crate::evidence::EvidenceStore;

#[test]
fn evidence_tracked_per_claim() {
    let mut store = EvidenceStore::new();
    store.add("claim-A", "source1".into());
    store.add("claim-A", "source2".into());
    assert_eq!(store.count("claim-A"), 2);
}

#[test]
fn evidence_returns_empty_for_unknown_claim() {
    let store = EvidenceStore::new();
    assert_eq!(store.get("unknown").len(), 0);
    assert_eq!(store.count("unknown"), 0);
}

#[test]
fn evidence_sources_retrievable() {
    let mut store = EvidenceStore::new();
    store.add("claim", "source-x".into());
    let sources = store.get("claim");
    assert_eq!(sources, &["source-x"]);
}

#[test]
fn evidence_isolated_per_claim() {
    let mut store = EvidenceStore::new();
    store.add("claim-A", "src-a".into());
    store.add("claim-B", "src-b".into());
    assert_eq!(store.count("claim-A"), 1);
    assert_eq!(store.count("claim-B"), 1);
}
