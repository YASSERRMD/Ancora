use crate::store::MemoryStore;
use crate::entry::MemoryKind;

/// Prunes entries below a score threshold or older than a cutoff.
pub fn prune_by_age(store: &mut MemoryStore, cutoff_secs: u64, now: u64) -> usize {
    let to_remove: Vec<String> = store
        .by_kind(&MemoryKind::Fact)
        .iter()
        .chain(store.by_kind(&MemoryKind::Context).iter())
        .chain(store.by_kind(&MemoryKind::Preference).iter())
        .chain(store.by_kind(&MemoryKind::Instruction).iter())
        .filter(|e| now.saturating_sub(e.last_accessed) > cutoff_secs)
        .map(|e| e.id.clone())
        .collect();

    let count = to_remove.len();
    for id in to_remove {
        store.remove(&id);
    }
    count
}
