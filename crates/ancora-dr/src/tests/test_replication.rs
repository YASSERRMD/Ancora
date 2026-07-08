#[cfg(test)]
mod tests {
    use crate::{replicate, replication_lag, JournalEntry, JournalStore};

    fn populated_primary(n: u64) -> JournalStore {
        let mut s = JournalStore::new();
        for i in 1..=n {
            s.append(JournalEntry {
                seq: i,
                data: format!("d{}", i),
            })
            .unwrap();
        }
        s
    }

    #[test]
    fn replication_keeps_secondary_current() {
        let primary = populated_primary(5);
        let mut secondary = JournalStore::new();
        replicate(&primary, &mut secondary);
        assert_eq!(secondary.entries.len(), 5);
        assert_eq!(replication_lag(&primary, &secondary), 0);
    }

    #[test]
    fn replication_lag_measured() {
        let primary = populated_primary(5);
        let secondary = populated_primary(3);
        let lag = replication_lag(&primary, &secondary);
        assert_eq!(lag, 2);
    }

    #[test]
    fn incremental_replication_only_syncs_new_entries() {
        let mut primary = populated_primary(3);
        let mut secondary = JournalStore::new();
        replicate(&primary, &mut secondary);
        // Add more to primary
        primary
            .append(JournalEntry {
                seq: 4,
                data: "d4".into(),
            })
            .unwrap();
        let synced = replicate(&primary, &mut secondary);
        assert_eq!(synced, 1);
        assert_eq!(secondary.entries.len(), 4);
    }
}
