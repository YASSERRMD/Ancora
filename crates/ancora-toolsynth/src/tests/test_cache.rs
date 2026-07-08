use crate::cache::SynthCache;
use crate::spec::{spec_from_goal, EffectClass, ToolSpec};
use serde_json::json;

#[test]
fn cached_tool_reused() {
    let mut cache = SynthCache::default();
    let spec = spec_from_goal("List files");
    cache.insert("List files", spec);
    assert!(cache.get("List files").is_some());
}

#[test]
fn cache_miss_returns_none() {
    let cache = SynthCache::default();
    assert!(cache.get("some goal").is_none());
}

#[test]
fn cache_len_tracks_entries() {
    let mut cache = SynthCache::default();
    cache.insert(
        "g1",
        ToolSpec::new(
            "t1",
            "d",
            json!({ "type": "object" }),
            EffectClass::ReadOnly,
        ),
    );
    cache.insert(
        "g2",
        ToolSpec::new(
            "t2",
            "d",
            json!({ "type": "object" }),
            EffectClass::ReadOnly,
        ),
    );
    assert_eq!(cache.len(), 2);
}
