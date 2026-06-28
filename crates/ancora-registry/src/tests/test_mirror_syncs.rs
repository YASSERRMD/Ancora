use crate::mirror::{MirrorEntry, MirrorSnapshot, MirrorStore, sync_from_snapshot};
use crate::versioning::Version;

#[test]
fn applying_snapshot_populates_mirror() {
    let mut store = MirrorStore::default();
    let mut snapshot = MirrorSnapshot::default();
    snapshot.add(MirrorEntry::new("tool-a", Version::new(1, 0, 0), b"data-a".to_vec()));
    snapshot.add(MirrorEntry::new("tool-b", Version::new(2, 1, 0), b"data-b".to_vec()));

    store.apply_snapshot(&snapshot);

    assert_eq!(store.entry_count(), 2);
    assert_eq!(
        store.get("tool-a", &Version::new(1, 0, 0)),
        Some(&b"data-a".to_vec())
    );
}

#[test]
fn sync_returns_count_of_new_entries() {
    let mut store = MirrorStore::default();
    let mut snapshot = MirrorSnapshot::default();
    snapshot.add(MirrorEntry::new("tool-c", Version::new(1, 0, 0), b"c".to_vec()));

    let added = sync_from_snapshot(&mut store, &snapshot);
    assert_eq!(added, 1);
}

#[test]
fn second_sync_with_same_snapshot_adds_zero() {
    let mut store = MirrorStore::default();
    let mut snapshot = MirrorSnapshot::default();
    snapshot.add(MirrorEntry::new("tool-d", Version::new(1, 0, 0), b"d".to_vec()));

    sync_from_snapshot(&mut store, &snapshot);
    let added = sync_from_snapshot(&mut store, &snapshot);
    // HashMap insert overwrites; count stays the same.
    assert_eq!(added, 0);
}

#[test]
fn mirror_missing_entry_returns_none() {
    let store = MirrorStore::default();
    assert!(store.get("ghost", &Version::new(1, 0, 0)).is_none());
}

#[test]
fn snapshot_round_trips_through_store() {
    let mut store = MirrorStore::default();
    let mut original = MirrorSnapshot::default();
    original.add(MirrorEntry::new("tool-e", Version::new(3, 0, 0), b"e-data".to_vec()));
    store.apply_snapshot(&original);

    let exported = store.to_snapshot();
    assert_eq!(exported.len(), 1);
    assert_eq!(exported.entries()[0].name, "tool-e");
}
