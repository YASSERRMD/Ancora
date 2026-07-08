use crate::feed::{FeedFormat, FeedStore, ThreatFeed};

#[test]
fn feed_register_and_get() {
    let mut store = FeedStore::new();
    let feed = ThreatFeed::new(
        "f1",
        "t1",
        "Test Feed",
        FeedFormat::Internal,
        "internal://x",
        1,
    );
    store.register_feed(feed);
    assert!(store.get_feed("f1").is_some());
    assert_eq!(store.feed_count(), 1);
}

#[test]
fn feed_add_indicators() {
    let mut store = FeedStore::new();
    store.register_feed(ThreatFeed::new("f1", "t1", "F", FeedFormat::Json, "x", 1));
    store.add_indicator_to_feed("f1", "i1");
    store.add_indicator_to_feed("f1", "i2");
    assert_eq!(store.indicators_for_feed("f1").len(), 2);
    assert_eq!(store.indicators_for_feed("f2").len(), 0);
}

#[test]
fn feed_disable() {
    let mut store = FeedStore::new();
    let mut feed = ThreatFeed::new("f1", "t1", "F", FeedFormat::Stix, "x", 1);
    feed.disable();
    store.register_feed(feed);
    assert_eq!(store.enabled_feeds().len(), 0);
}

#[test]
fn feed_for_tenant() {
    let mut store = FeedStore::new();
    store.register_feed(ThreatFeed::new("f1", "t1", "A", FeedFormat::Csv, "x", 1));
    store.register_feed(ThreatFeed::new("f2", "t2", "B", FeedFormat::Json, "y", 1));
    assert_eq!(store.for_tenant("t1").len(), 1);
}
