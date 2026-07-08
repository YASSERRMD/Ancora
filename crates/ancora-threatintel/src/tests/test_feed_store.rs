use crate::feed::{FeedFormat, FeedStore, ThreatFeed};

#[test]
fn feed_store_enabled_feeds() {
    let mut store = FeedStore::new();
    store.register_feed(ThreatFeed::new(
        "f1",
        "t1",
        "A",
        FeedFormat::Internal,
        "x",
        1,
    ));
    let mut f2 = ThreatFeed::new("f2", "t1", "B", FeedFormat::Json, "y", 1);
    f2.disable();
    store.register_feed(f2);
    assert_eq!(store.enabled_feeds().len(), 1);
    assert_eq!(store.feed_count(), 2);
}

#[test]
fn feed_store_update_tick() {
    let mut store = FeedStore::new();
    store.register_feed(ThreatFeed::new("f1", "t1", "A", FeedFormat::Stix, "x", 1));
    if let Some(f) = store.get_feed_mut("f1") {
        f.update_tick(500);
    }
    assert_eq!(store.get_feed("f1").map(|f| f.last_updated_tick), Some(500));
}
