#[cfg(test)]
mod tests {
    use crate::{drill::run_drill, JournalEntry, JournalStore};

    fn synced_pair(n: u64) -> (JournalStore, JournalStore) {
        let mut p = JournalStore::new();
        for i in 1..=n {
            p.append(JournalEntry {
                seq: i,
                data: format!("d{}", i),
            })
            .unwrap();
        }
        let mut s = JournalStore::new();
        crate::replicate(&p, &mut s);
        (p, s)
    }

    #[test]
    fn drill_completes_within_rto() {
        let (mut p, mut s) = synced_pair(3);
        let result = run_drill(&mut p, &mut s, 300, 0);
        assert!(result.passed, "drill must complete within RTO");
        assert_eq!(result.entries_lost, 0);
    }

    #[test]
    fn drill_records_failover_time() {
        let (mut p, mut s) = synced_pair(2);
        let result = run_drill(&mut p, &mut s, 300, 0);
        assert!(result.failover_secs > 0);
        assert!(result.failback_secs > 0);
    }
}
