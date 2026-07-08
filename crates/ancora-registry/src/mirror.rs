use std::collections::HashMap;

use crate::versioning::Version;

/// A single entry in a mirror snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorEntry {
    pub name: String,
    pub version: Version,
    pub payload: Vec<u8>,
}

impl MirrorEntry {
    pub fn new(name: impl Into<String>, version: Version, payload: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            version,
            payload,
        }
    }
}

/// A snapshot of entries exported from an upstream registry for local mirroring.
#[derive(Debug, Default, Clone)]
pub struct MirrorSnapshot {
    entries: Vec<MirrorEntry>,
}

impl MirrorSnapshot {
    pub fn add(&mut self, entry: MirrorEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[MirrorEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// An in-memory mirror store that holds a local copy of upstream entries.
#[derive(Debug, Default)]
pub struct MirrorStore {
    data: HashMap<(String, Version), Vec<u8>>,
}

impl MirrorStore {
    /// Apply a snapshot, inserting all entries into the local mirror.
    pub fn apply_snapshot(&mut self, snapshot: &MirrorSnapshot) {
        for entry in snapshot.entries() {
            self.data.insert(
                (entry.name.clone(), entry.version.clone()),
                entry.payload.clone(),
            );
        }
    }

    /// Look up a mirrored entry.
    pub fn get(&self, name: &str, version: &Version) -> Option<&Vec<u8>> {
        self.data.get(&(name.to_string(), version.clone()))
    }

    /// Return the number of entries held in the mirror.
    pub fn entry_count(&self) -> usize {
        self.data.len()
    }

    /// Produce a snapshot of all currently mirrored entries.
    pub fn to_snapshot(&self) -> MirrorSnapshot {
        let mut snap = MirrorSnapshot::default();
        for ((name, version), payload) in &self.data {
            snap.add(MirrorEntry::new(
                name.clone(),
                version.clone(),
                payload.clone(),
            ));
        }
        snap
    }
}

/// Sync a local mirror from a snapshot, returning the count of new entries added.
pub fn sync_from_snapshot(store: &mut MirrorStore, snapshot: &MirrorSnapshot) -> usize {
    let before = store.entry_count();
    store.apply_snapshot(snapshot);
    store.entry_count() - before
}
