use crate::citation::CitationStore;

#[test]
fn citations_attached_to_claims() {
    let mut store = CitationStore::new();
    store.add("claim-A", "ref1".into());
    assert!(store.has_citations("claim-A"));
    assert_eq!(store.get("claim-A"), &["ref1"]);
}

#[test]
fn no_citations_for_unknown_claim() {
    let store = CitationStore::new();
    assert!(!store.has_citations("unknown"));
    assert!(store.get("unknown").is_empty());
}

#[test]
fn multiple_citations_per_claim() {
    let mut store = CitationStore::new();
    store.add("claim", "ref1".into());
    store.add("claim", "ref2".into());
    assert_eq!(store.get("claim").len(), 2);
}

#[test]
fn all_cited_claims_lists_keys() {
    let mut store = CitationStore::new();
    store.add("claim-A", "ref-a".into());
    store.add("claim-B", "ref-b".into());
    let keys = store.all_cited_claims();
    assert_eq!(keys.len(), 2);
}
