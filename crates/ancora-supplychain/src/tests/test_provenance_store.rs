#[cfg(test)]
mod tests {
    use crate::provenance::{ProvenanceKind, ProvenanceRecord, ProvenanceStore};

    fn make_record(component_id: &str) -> ProvenanceRecord {
        ProvenanceRecord::new(
            component_id,
            ProvenanceKind::BuildSystem,
            "ci.example.com",
            "build-1",
            800,
        )
    }

    #[test]
    fn test_new_store_has_zero_count() {
        let store = ProvenanceStore::new();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_record_increments_count() {
        let mut store = ProvenanceStore::new();
        store.record(make_record("comp-1"));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_get_returns_recorded_entry() {
        let mut store = ProvenanceStore::new();
        store.record(make_record("comp-2"));
        let result = store.get("comp-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap().component_id, "comp-2");
    }

    #[test]
    fn test_get_returns_none_for_missing() {
        let store = ProvenanceStore::new();
        assert!(store.get("never-recorded").is_none());
    }

    #[test]
    fn test_has_provenance_true_after_record() {
        let mut store = ProvenanceStore::new();
        store.record(make_record("comp-3"));
        assert!(store.has_provenance("comp-3"));
    }

    #[test]
    fn test_has_provenance_false_for_missing() {
        let store = ProvenanceStore::new();
        assert!(!store.has_provenance("comp-x"));
    }

    #[test]
    fn test_count_multiple_records() {
        let mut store = ProvenanceStore::new();
        store.record(make_record("c1"));
        store.record(make_record("c2"));
        store.record(make_record("c3"));
        assert_eq!(store.count(), 3);
    }
}
