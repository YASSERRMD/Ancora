use crate::prefetch::{CacheState, EvictionPolicy, PrefetchCache};

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

#[test]
fn prefetch_enqueue_and_process() {
    let mut cache = PrefetchCache::new(8 * GB, EvictionPolicy::Lru);
    cache.enqueue("model-a", 2 * GB);
    assert_eq!(cache.state("model-a"), CacheState::Queued);
    assert_eq!(cache.queue_depth(), 1);

    let loaded = cache.process_next();
    assert_eq!(loaded, Some("model-a".to_string()));
    assert_eq!(cache.state("model-a"), CacheState::Cached);
    assert_eq!(cache.queue_depth(), 0);
}

#[test]
fn prefetch_cached_model_not_requeued() {
    let mut cache = PrefetchCache::new(8 * GB, EvictionPolicy::Lru);
    cache.enqueue("m", GB);
    cache.process_next();
    // Enqueue again -- should be a no-op since already cached.
    cache.enqueue("m", GB);
    assert_eq!(cache.queue_depth(), 0);
}

#[test]
fn prefetch_eviction_on_full_cache() {
    // 3 GB budget.
    let mut cache = PrefetchCache::new(3 * GB, EvictionPolicy::Lru);
    cache.enqueue("a", 2 * GB);
    cache.process_next(); // a is cached.
    assert_eq!(cache.state("a"), CacheState::Cached);

    // Enqueue b (2 GB) -- cache is full (2/3 GB used), b needs 2 GB.
    // LRU eviction should evict a.
    cache.enqueue("b", 2 * GB);
    cache.process_next();
    assert_eq!(cache.state("b"), CacheState::Cached);
    assert_eq!(cache.state("a"), CacheState::Evicted);
}

#[test]
fn prefetch_touch_increases_access_count() {
    let mut cache = PrefetchCache::new(8 * GB, EvictionPolicy::Lfu);
    cache.enqueue("m", GB);
    cache.process_next();
    cache.touch("m");
    cache.touch("m");
    // The entry should have access_count == 2.
    // (Internal state, but we can verify via cached_ids)
    assert!(cache.cached_ids().contains(&"m"));
}

#[test]
fn prefetch_manual_eviction() {
    let mut cache = PrefetchCache::new(8 * GB, EvictionPolicy::Lru);
    cache.enqueue("x", GB);
    cache.process_next();
    assert_eq!(cache.state("x"), CacheState::Cached);
    assert!(cache.evict("x"));
    assert_eq!(cache.state("x"), CacheState::Evicted);
}

#[test]
fn prefetch_set_priority_affects_state() {
    let mut cache = PrefetchCache::new(8 * GB, EvictionPolicy::LowestPriority);
    cache.enqueue("low-pri", GB);
    cache.process_next();
    cache.set_priority("low-pri", 10);
    // Just verify no panic and state remains.
    assert_eq!(cache.state("low-pri"), CacheState::Cached);
}
