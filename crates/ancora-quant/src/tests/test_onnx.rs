use crate::onnx::{ExecutionProvider, OnnxDescriptor, OnnxPrecision};

#[test]
fn onnx_descriptor_loads() {
    let desc = OnnxDescriptor::new(
        "bert-base-int8",
        "/tmp/bert-base-int8.onnx",
        17,
        OnnxPrecision::Int8,
        350_000_000,
        0.11,
    );
    assert_eq!(desc.name, "bert-base-int8");
    assert_eq!(desc.opset_version, 17);
    assert_eq!(desc.precision, OnnxPrecision::Int8);
    assert_eq!(desc.extension(), "onnx");
    assert!(!desc.providers.is_empty());
}

#[test]
fn onnx_precision_from_tag() {
    assert_eq!(OnnxPrecision::from_tag("fp32"), OnnxPrecision::Float32);
    assert_eq!(OnnxPrecision::from_tag("fp16"), OnnxPrecision::Float16);
    assert_eq!(OnnxPrecision::from_tag("int8"), OnnxPrecision::Int8);
    assert_eq!(OnnxPrecision::from_tag("int4"), OnnxPrecision::Int4);
    assert_eq!(OnnxPrecision::from_tag("bf16"), OnnxPrecision::BFloat16);
}

#[test]
fn onnx_precision_bits() {
    assert_eq!(OnnxPrecision::Float32.bits(), 32);
    assert_eq!(OnnxPrecision::Float16.bits(), 16);
    assert_eq!(OnnxPrecision::Int8.bits(), 8);
    assert_eq!(OnnxPrecision::Int4.bits(), 4);
}

#[test]
fn onnx_estimated_ram_decreases_with_lower_precision() {
    let make = |p: OnnxPrecision| {
        OnnxDescriptor::new("m", "/tmp/m.onnx", 17, p, 0, 7.0).estimated_ram_bytes()
    };
    assert!(make(OnnxPrecision::Float32) > make(OnnxPrecision::Float16));
    assert!(make(OnnxPrecision::Float16) > make(OnnxPrecision::Int8));
    assert!(make(OnnxPrecision::Int8) > make(OnnxPrecision::Int4));
}

#[test]
fn onnx_with_provider_and_metadata() {
    let desc = OnnxDescriptor::new("m", "/tmp/m.onnx", 17, OnnxPrecision::Int8, 0, 0.11)
        .with_provider(ExecutionProvider::Cuda)
        .with_max_sequence_length(512)
        .with_metadata("task", "classification");
    assert!(desc.providers.contains(&ExecutionProvider::Cuda));
    assert_eq!(desc.max_sequence_length, Some(512));
    assert_eq!(
        desc.metadata.get("task").map(|s| s.as_str()),
        Some("classification")
    );
}
