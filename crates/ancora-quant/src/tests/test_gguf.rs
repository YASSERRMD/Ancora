use crate::gguf::{GgufDescriptor, GgufQuantType};

#[test]
fn gguf_descriptor_loads() {
    let desc = GgufDescriptor::new(
        "llama3-8b-q4",
        "/tmp/llama3-8b-q4.gguf",
        "llama",
        8.0,
        GgufQuantType::Q4_K,
        4_500_000_000,
        4096,
    );
    assert_eq!(desc.name, "llama3-8b-q4");
    assert_eq!(desc.architecture, "llama");
    assert_eq!(desc.param_count_billions, 8.0);
    assert_eq!(desc.context_length, 4096);
    assert_eq!(desc.extension(), "gguf");
}

#[test]
fn gguf_quant_type_from_tag() {
    assert_eq!(GgufQuantType::from_tag("Q4_K_M"), GgufQuantType::Q4_K);
    assert_eq!(GgufQuantType::from_tag("Q5_K_S"), GgufQuantType::Q5_K);
    assert_eq!(GgufQuantType::from_tag("Q8_0"), GgufQuantType::Q8_0);
    assert_eq!(GgufQuantType::from_tag("F16"), GgufQuantType::F16);
    assert_eq!(GgufQuantType::from_tag("BF16"), GgufQuantType::BF16);
}

#[test]
fn gguf_bits_per_weight() {
    assert_eq!(GgufQuantType::F32.bits_per_weight(), 32.0);
    assert_eq!(GgufQuantType::F16.bits_per_weight(), 16.0);
    assert_eq!(GgufQuantType::Q8_0.bits_per_weight(), 8.0);
    assert!(GgufQuantType::Q4_K.bits_per_weight() < 5.0);
    assert!(GgufQuantType::Q2_K.bits_per_weight() < 3.0);
}

#[test]
fn gguf_estimated_ram_increases_with_quant() {
    let make = |q: GgufQuantType| {
        GgufDescriptor::new("m", "/tmp/m.gguf", "arch", 7.0, q, 0, 2048).estimated_ram_bytes()
    };
    let f32_ram = make(GgufQuantType::F32);
    let q8_ram = make(GgufQuantType::Q8_0);
    let q4_ram = make(GgufQuantType::Q4_K);
    assert!(f32_ram > q8_ram);
    assert!(q8_ram > q4_ram);
}

#[test]
fn gguf_metadata_roundtrip() {
    let desc = GgufDescriptor::new(
        "m",
        "/tmp/m.gguf",
        "mistral",
        7.0,
        GgufQuantType::Q5_K,
        0,
        2048,
    )
    .with_metadata("source", "huggingface")
    .with_metadata("license", "llama3");
    assert_eq!(
        desc.metadata.get("source").map(|s| s.as_str()),
        Some("huggingface")
    );
    assert_eq!(
        desc.metadata.get("license").map(|s| s.as_str()),
        Some("llama3")
    );
}
