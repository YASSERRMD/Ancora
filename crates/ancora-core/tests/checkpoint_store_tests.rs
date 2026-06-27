use ancora_core::journal::{CheckpointStore, MemoryStore};

#[test]
fn load_checkpoint_returns_none_when_no_checkpoint_saved() {
    let store = MemoryStore::new();
    let result = store.load_checkpoint("run-new").unwrap();
    assert!(result.is_none(), "must return None when no checkpoint has been saved");
}

#[test]
fn save_and_load_checkpoint_round_trips() {
    let store = MemoryStore::new();
    let data = b"checkpoint-payload-v1".to_vec();
    store.save("run-cp", 5, &data).unwrap();

    let loaded = store.load_checkpoint("run-cp").unwrap();
    assert!(loaded.is_some(), "must return Some after save");
    let (at_seq, loaded_data) = loaded.unwrap();
    assert_eq!(at_seq, 5);
    assert_eq!(loaded_data, data);
}

#[test]
fn save_overwrites_previous_checkpoint() {
    let store = MemoryStore::new();
    store.save("run-ow", 3, b"v1").unwrap();
    store.save("run-ow", 7, b"v2").unwrap();

    let (at_seq, loaded_data) = store.load_checkpoint("run-ow").unwrap().unwrap();
    assert_eq!(at_seq, 7, "latest checkpoint must overwrite the earlier one");
    assert_eq!(loaded_data, b"v2");
}

#[test]
fn checkpoint_for_different_run_ids_are_independent() {
    let store = MemoryStore::new();
    store.save("run-a", 1, b"payload-a").unwrap();
    store.save("run-b", 2, b"payload-b").unwrap();

    let (_, a) = store.load_checkpoint("run-a").unwrap().unwrap();
    let (_, b) = store.load_checkpoint("run-b").unwrap().unwrap();
    assert_eq!(a, b"payload-a");
    assert_eq!(b, b"payload-b");
}

#[test]
fn empty_payload_checkpoint_is_accepted() {
    let store = MemoryStore::new();
    store.save("run-empty", 0, b"").unwrap();
    let (_, data) = store.load_checkpoint("run-empty").unwrap().unwrap();
    assert!(data.is_empty());
}

#[test]
fn large_payload_round_trips_intact() {
    let store = MemoryStore::new();
    let payload: Vec<u8> = (0u8..=255).cycle().take(64 * 1024).collect();
    store.save("run-large", 100, &payload).unwrap();
    let (at_seq, loaded) = store.load_checkpoint("run-large").unwrap().unwrap();
    assert_eq!(at_seq, 100);
    assert_eq!(loaded, payload);
}

#[test]
fn checkpoint_at_seq_zero_is_accepted() {
    let store = MemoryStore::new();
    store.save("run-z", 0, b"boot").unwrap();
    let (at_seq, _) = store.load_checkpoint("run-z").unwrap().unwrap();
    assert_eq!(at_seq, 0);
}
