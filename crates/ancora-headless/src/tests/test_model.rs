use crate::model::{
    total_model_memory_mb, validate_checksum, ModelDescriptor, ModelRegistry, PreloadState,
    Quantization,
};

#[test]
fn test_model_preloads() {
    let mut registry = ModelRegistry::new();
    let desc = ModelDescriptor::new("agent-q4", "/opt/models/agent-q4.gguf", 4 * 1024 * 1024 * 1024);
    registry.register(desc);
    let records = registry.preload_all();
    assert_eq!(records.len(), 1);
    assert!(registry.all_loaded());
}

#[test]
fn test_model_state_transitions() {
    let mut registry = ModelRegistry::new();
    let desc = ModelDescriptor::new("m1", "/models/m1.gguf", 1024 * 1024 * 1024);
    registry.register(desc);
    assert_eq!(*registry.state("m1").unwrap(), PreloadState::Pending);
    registry.set_state("m1", PreloadState::Loaded);
    assert_eq!(*registry.state("m1").unwrap(), PreloadState::Loaded);
}

#[test]
fn test_model_size_mb() {
    let desc = ModelDescriptor::new("m", "/m.gguf", 2 * 1024 * 1024 * 1024);
    assert_eq!(desc.size_mb(), 2048);
}

#[test]
fn test_model_loaded_count() {
    let mut registry = ModelRegistry::new();
    for i in 0..3 {
        let d = ModelDescriptor::new(format!("m{}", i), format!("/m{}.gguf", i), 1024 * 1024);
        registry.register(d);
    }
    registry.preload_all();
    assert_eq!(registry.loaded_count(), 3);
}

#[test]
fn test_validate_checksum_match() {
    let desc = ModelDescriptor::new("m", "/m.gguf", 0)
        .with_sha256("abc123");
    assert!(validate_checksum(&desc, "abc123"));
}

#[test]
fn test_validate_checksum_mismatch() {
    let desc = ModelDescriptor::new("m", "/m.gguf", 0)
        .with_sha256("abc123");
    assert!(!validate_checksum(&desc, "wrong"));
}

#[test]
fn test_validate_checksum_none_always_passes() {
    let desc = ModelDescriptor::new("m", "/m.gguf", 0);
    assert!(validate_checksum(&desc, "anything"));
}

#[test]
fn test_total_model_memory() {
    use crate::model::PreloadRecord;
    use std::time::Duration;
    let records = vec![
        PreloadRecord::success("m1", Duration::ZERO, 1024),
        PreloadRecord::success("m2", Duration::ZERO, 512),
    ];
    assert_eq!(total_model_memory_mb(&records), 1536);
}

#[test]
fn test_quantization_display() {
    assert_eq!(Quantization::Q4.to_string(), "q4");
    assert_eq!(Quantization::F16.to_string(), "f16");
}
