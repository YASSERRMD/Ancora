use crate::gguf::{GgufDescriptor, GgufQuantType};
use crate::registry::ModelEntry;
use crate::runtime::{LoadError, RuntimeManager};

fn small_entry() -> ModelEntry {
    ModelEntry::Gguf(GgufDescriptor::new(
        "small",
        "/tmp/small.gguf",
        "llama",
        3.0,
        GgufQuantType::Q4_K,
        0,
        2048,
    ))
}

fn large_entry() -> ModelEntry {
    ModelEntry::Gguf(GgufDescriptor::new(
        "large",
        "/tmp/large.gguf",
        "llama",
        70.0,
        GgufQuantType::Q4_K,
        0,
        2048,
    ))
}

#[test]
fn load_and_unload_works() {
    // 8 GB total RAM.
    let mut mgr = RuntimeManager::new(8 * 1024 * 1024 * 1024);
    let entry = small_entry();
    let handle = mgr.load("small", &entry).expect("load should succeed");
    assert!(mgr.is_loaded("small"));
    assert_eq!(mgr.loaded_count(), 1);

    let unloaded = mgr.unload(handle).expect("unload should succeed");
    assert_eq!(unloaded.model_id, "small");
    assert!(!mgr.is_loaded("small"));
    assert_eq!(mgr.loaded_count(), 0);
}

#[test]
fn load_fails_when_out_of_memory() {
    // 1 MB total RAM -- too small for any model.
    let mut mgr = RuntimeManager::new(1024 * 1024);
    let entry = large_entry();
    let result = mgr.load("large", &entry);
    assert!(matches!(result, Err(LoadError::OutOfMemory { .. })));
}

#[test]
fn available_ram_decreases_after_load() {
    let total = 8 * 1024 * 1024 * 1024_u64;
    let mut mgr = RuntimeManager::new(total);
    let entry = small_entry();
    let before = mgr.available_ram();
    mgr.load("small", &entry).unwrap();
    let after = mgr.available_ram();
    assert!(after < before);
}

#[test]
fn available_ram_restored_after_unload() {
    let total = 8 * 1024 * 1024 * 1024_u64;
    let mut mgr = RuntimeManager::new(total);
    let entry = small_entry();
    let before = mgr.available_ram();
    let handle = mgr.load("small", &entry).unwrap();
    mgr.unload(handle).unwrap();
    assert_eq!(mgr.available_ram(), before);
}

#[test]
fn unload_all_by_id_works() {
    let mut mgr = RuntimeManager::new(64 * 1024 * 1024 * 1024_u64);
    let entry = small_entry();
    mgr.load("small", &entry).unwrap();
    mgr.load("small", &entry).unwrap();
    assert_eq!(mgr.loaded_count(), 2);
    let unloaded = mgr.unload_all_by_id("small");
    assert_eq!(unloaded, 2);
    assert!(!mgr.is_loaded("small"));
}
