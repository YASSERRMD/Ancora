use crate::gguf::{GgufDescriptor, GgufQuantType};
use crate::onnx::{OnnxDescriptor, OnnxPrecision};
use crate::registry::ModelRegistry;

fn sample_gguf(name: &str, params: f32, quant: GgufQuantType) -> GgufDescriptor {
    GgufDescriptor::new(name, format!("/tmp/{}.gguf", name), "llama", params, quant, 0, 2048)
}

fn sample_onnx(name: &str, params: f32) -> OnnxDescriptor {
    OnnxDescriptor::new(name, format!("/tmp/{}.onnx", name), 17, OnnxPrecision::Int8, 0, params)
}

#[test]
fn registry_lists_local_models() {
    let mut reg = ModelRegistry::new();
    reg.register_gguf("llama-7b-q4", sample_gguf("llama-7b-q4", 7.0, GgufQuantType::Q4_K));
    reg.register_gguf("llama-7b-q8", sample_gguf("llama-7b-q8", 7.0, GgufQuantType::Q8_0));
    reg.register_onnx("bert-int8", sample_onnx("bert-int8", 0.11));

    assert_eq!(reg.len(), 3);
    assert!(!reg.is_empty());
    let ids: Vec<&str> = reg.ids().collect();
    assert!(ids.contains(&"llama-7b-q4"));
    assert!(ids.contains(&"bert-int8"));
}

#[test]
fn registry_get_returns_entry() {
    let mut reg = ModelRegistry::new();
    reg.register_gguf("m", sample_gguf("m", 7.0, GgufQuantType::Q4_K));
    assert!(reg.get("m").is_some());
    assert!(reg.get("nonexistent").is_none());
}

#[test]
fn registry_remove_works() {
    let mut reg = ModelRegistry::new();
    reg.register_gguf("x", sample_gguf("x", 7.0, GgufQuantType::Q4_0));
    assert_eq!(reg.len(), 1);
    reg.remove("x");
    assert_eq!(reg.len(), 0);
    assert!(reg.is_empty());
}

#[test]
fn registry_list_by_ram_sorted() {
    let mut reg = ModelRegistry::new();
    reg.register_gguf("big", sample_gguf("big", 70.0, GgufQuantType::F16));
    reg.register_gguf("small", sample_gguf("small", 3.0, GgufQuantType::Q4_K));
    reg.register_gguf("medium", sample_gguf("medium", 13.0, GgufQuantType::Q8_0));

    let sorted = reg.list_by_ram();
    let rams: Vec<u64> = sorted.iter().map(|(_, e)| e.estimated_ram_bytes()).collect();
    for i in 1..rams.len() {
        assert!(rams[i] >= rams[i - 1]);
    }
}

#[test]
fn registry_models_fitting_ram() {
    let mut reg = ModelRegistry::new();
    reg.register_gguf("small", sample_gguf("small", 3.0, GgufQuantType::Q4_K));
    reg.register_gguf("large", sample_gguf("large", 70.0, GgufQuantType::Q4_K));

    let budget = 4 * 1024 * 1024 * 1024_u64; // 4 GB
    let fitting = reg.models_fitting_ram(budget);
    // Small 3B Q4 should fit; 70B Q4 won't.
    let ids: Vec<&str> = fitting.iter().map(|(id, _)| *id).collect();
    assert!(ids.contains(&"small"));
    assert!(!ids.contains(&"large"));
}
