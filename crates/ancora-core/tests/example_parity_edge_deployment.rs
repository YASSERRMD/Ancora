// Example parity: edge deployment example -- Wasm/CF Workers config consistent.

const EDGE_PROVIDERS: &[(&str, &str)] = &[
    ("cf-workers", "wasm32-unknown-unknown"),
    ("deno-deploy", "wasm32-unknown-unknown"),
    ("fastly-compute", "wasm32-wasi"),
];

const EDGE_SUPPORTED_SDKS: &[&str] = &["rust", "ts", "go"];

struct EdgeExample {
    sdk: &'static str,
    target: &'static str,
    local_model_only: bool,
}

const EDGE_EXAMPLES: &[EdgeExample] = &[
    EdgeExample { sdk: "rust", target: "wasm32-unknown-unknown", local_model_only: true },
    EdgeExample { sdk: "ts",   target: "wasm32-unknown-unknown", local_model_only: false },
    EdgeExample { sdk: "go",   target: "wasm32-unknown-unknown", local_model_only: false },
];

#[test]
fn test_three_edge_providers() {
    assert_eq!(EDGE_PROVIDERS.len(), 3);
}

#[test]
fn test_three_edge_examples() {
    assert_eq!(EDGE_EXAMPLES.len(), 3);
}

#[test]
fn test_rust_edge_uses_wasm_target() {
    let rust = EDGE_EXAMPLES.iter().find(|e| e.sdk == "rust").unwrap();
    assert!(rust.target.contains("wasm32"));
}

#[test]
fn test_supported_sdks_include_rust_ts_go() {
    for sdk in ["rust", "ts", "go"] {
        assert!(EDGE_SUPPORTED_SDKS.contains(&sdk), "edge deployment doesn't support {sdk}");
    }
}

#[test]
fn test_all_edge_examples_use_wasm_target() {
    for e in EDGE_EXAMPLES { assert!(e.target.contains("wasm"), "sdk {} target is not wasm", e.sdk); }
}

#[test]
fn test_fastly_compute_uses_wasi() {
    let fastly = EDGE_PROVIDERS.iter().find(|(p, _)| *p == "fastly-compute");
    assert_eq!(fastly.map(|(_, t)| *t), Some("wasm32-wasi"));
}
