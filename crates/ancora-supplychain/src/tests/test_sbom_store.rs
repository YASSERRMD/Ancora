#[cfg(test)]
mod tests {
    use crate::sbom::{Sbom, SbomFormat, SbomStore};

    fn make_sbom(id: &str, tenant_id: &str) -> Sbom {
        Sbom::new(id, tenant_id, SbomFormat::CycloneDx, 1000)
    }

    #[test]
    fn test_new_store_has_zero_count() {
        let store = SbomStore::new();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_insert_increments_count() {
        let mut store = SbomStore::new();
        store.insert(make_sbom("s1", "tenant-a"));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_get_returns_inserted_sbom() {
        let mut store = SbomStore::new();
        store.insert(make_sbom("s1", "tenant-a"));
        let result = store.get("s1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "s1");
    }

    #[test]
    fn test_get_returns_none_for_missing() {
        let store = SbomStore::new();
        assert!(store.get("nonexistent").is_none());
    }

    #[test]
    fn test_for_tenant_returns_only_matching_sboms() {
        let mut store = SbomStore::new();
        store.insert(make_sbom("s1", "tenant-a"));
        store.insert(make_sbom("s2", "tenant-a"));
        store.insert(make_sbom("s3", "tenant-b"));
        let tenant_sboms = store.for_tenant("tenant-a");
        assert_eq!(tenant_sboms.len(), 2);
    }

    #[test]
    fn test_for_tenant_returns_empty_for_unknown_tenant() {
        let mut store = SbomStore::new();
        store.insert(make_sbom("s1", "tenant-a"));
        assert!(store.for_tenant("tenant-z").is_empty());
    }

    #[test]
    fn test_count_multiple_inserts() {
        let mut store = SbomStore::new();
        store.insert(make_sbom("s1", "t1"));
        store.insert(make_sbom("s2", "t2"));
        store.insert(make_sbom("s3", "t3"));
        assert_eq!(store.count(), 3);
    }
}
