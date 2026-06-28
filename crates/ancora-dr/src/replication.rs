use crate::store::{JournalEntry, JournalStore};

/// Replication lag: number of entries the secondary is behind the primary.
pub fn replication_lag(primary: &JournalStore, secondary: &JournalStore) -> u64 {
    let primary_max = primary.max_seq();
    let secondary_max = secondary.max_seq();
    primary_max.saturating_sub(secondary_max)
}

/// Push all entries from primary to secondary that secondary hasn't seen yet.
pub fn replicate(primary: &JournalStore, secondary: &mut JournalStore) -> usize {
    let secondary_max = secondary.max_seq();
    let new_entries: Vec<JournalEntry> = primary.entries_since(secondary_max);
    let count = new_entries.len();
    for entry in new_entries {
        secondary.append(entry).ok();
    }
    count
}
