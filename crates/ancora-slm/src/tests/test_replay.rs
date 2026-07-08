use crate::replay::{make_replay_fn, ReplayModelFn, ReplayStore};

#[test]
fn test_replay_store_record_and_lookup() {
    let mut store = ReplayStore::new();
    store.record("hello prompt", "hello response");
    assert_eq!(store.lookup("hello prompt"), Some("hello response"));
    assert_eq!(store.lookup("unknown prompt"), None);
}

#[test]
fn test_replay_store_len() {
    let mut store = ReplayStore::new();
    assert_eq!(store.len(), 0);
    store.record("a", "b");
    assert_eq!(store.len(), 1);
}

#[test]
fn test_replay_store_serialisation_roundtrip() {
    let mut store = ReplayStore::new();
    store.record("p1", "r1");
    store.record("p2", "r2");

    let json = store.to_json().expect("should serialise");
    let restored = ReplayStore::from_json(&json).expect("should deserialise");

    assert_eq!(restored.lookup("p1"), Some("r1"));
    assert_eq!(restored.lookup("p2"), Some("r2"));
}

#[test]
fn test_replay_model_fn_returns_recorded_response() {
    let mut store = ReplayStore::new();
    store.record("What is Rust?", "A systems programming language.");
    let replay = ReplayModelFn::new(store, "default");
    assert_eq!(
        replay.call("What is Rust?"),
        "A systems programming language."
    );
}

#[test]
fn test_replay_model_fn_returns_default_for_unknown() {
    let store = ReplayStore::new();
    let replay = ReplayModelFn::new(store, "fallback");
    assert_eq!(replay.call("unknown prompt"), "fallback");
}

#[test]
fn test_make_replay_fn_deterministic() {
    let pairs = vec![
        ("prompt-a".to_string(), "response-a".to_string()),
        ("prompt-b".to_string(), "response-b".to_string()),
    ];
    let f = make_replay_fn(pairs, "default");
    // Same prompt always returns same response.
    assert_eq!(f("prompt-a"), "response-a");
    assert_eq!(f("prompt-a"), "response-a");
    assert_eq!(f("prompt-b"), "response-b");
    assert_eq!(f("unknown"), "default");
}

#[test]
fn test_replay_log_records_insertion_order() {
    let mut store = ReplayStore::new();
    store.record("first", "resp1");
    store.record("second", "resp2");
    let log = store.log();
    assert_eq!(log[0].prompt, "first");
    assert_eq!(log[1].prompt, "second");
}
