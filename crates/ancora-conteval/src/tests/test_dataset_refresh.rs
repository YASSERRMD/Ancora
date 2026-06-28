use crate::refresh::{DatasetEntry, DatasetRefresher, RefreshPolicy};
use std::time::{Duration, SystemTime};

fn make_entry(id: &str, ingested_at: SystemTime) -> DatasetEntry {
    DatasetEntry::new(id, ingested_at, format!("payload for {}", id))
}

#[test]
fn test_new_entries_are_not_stale() {
    let policy = RefreshPolicy::new(3600, 1, 100);
    let mut refresher = DatasetRefresher::new(policy);
    let now = SystemTime::now();
    refresher.ingest(make_entry("e1", now));
    assert_eq!(refresher.len(), 1);
    let evicted = refresher.evict_stale(now);
    assert_eq!(evicted, 0);
    assert_eq!(refresher.len(), 1);
}

#[test]
fn test_stale_entries_are_evicted() {
    let policy = RefreshPolicy::new(60, 1, 100);
    let mut refresher = DatasetRefresher::new(policy);
    let old_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1000);
    let now = old_time + Duration::from_secs(3600); // 1 hour later
    refresher.ingest(make_entry("old", old_time));
    refresher.ingest(make_entry("fresh", now));
    let evicted = refresher.evict_stale(now);
    assert_eq!(evicted, 1);
    assert_eq!(refresher.len(), 1);
}

#[test]
fn test_needs_refresh_when_below_min_fresh() {
    let policy = RefreshPolicy::new(60, 5, 100);
    let mut refresher = DatasetRefresher::new(policy);
    let now = SystemTime::now();
    refresher.ingest(make_entry("a", now));
    // Only 1 fresh sample, minimum is 5.
    assert!(refresher.needs_refresh(now));
}

#[test]
fn test_does_not_need_refresh_when_enough_fresh() {
    let policy = RefreshPolicy::new(3600, 2, 100);
    let mut refresher = DatasetRefresher::new(policy);
    let now = SystemTime::now();
    refresher.ingest(make_entry("a", now));
    refresher.ingest(make_entry("b", now));
    assert!(!refresher.needs_refresh(now));
}

#[test]
fn test_refresh_records_timestamp() {
    let policy = RefreshPolicy::new(60, 1, 100);
    let mut refresher = DatasetRefresher::new(policy);
    assert!(refresher.last_refresh().is_none());
    let now = SystemTime::now();
    refresher.refresh(now);
    assert!(refresher.last_refresh().is_some());
}

#[test]
fn test_refresh_trims_to_max_size() {
    let policy = RefreshPolicy::new(3600, 1, 3);
    let mut refresher = DatasetRefresher::new(policy);
    let now = SystemTime::now();
    for i in 0..6u32 {
        refresher.ingest(make_entry(&i.to_string(), now));
    }
    refresher.refresh(now);
    assert!(refresher.len() <= 3);
}

#[test]
fn test_entry_staleness_check() {
    let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(500);
    let entry = make_entry("x", t0);
    let t1 = t0 + Duration::from_secs(100);
    assert!(!entry.is_stale(t1, Duration::from_secs(200)));
    assert!(entry.is_stale(t1, Duration::from_secs(50)));
}
