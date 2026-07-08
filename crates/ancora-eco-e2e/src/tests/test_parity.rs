use crate::parity_e2e::{check_cross_runtime_parity, ExtensionCapabilities, LanguageRuntime};

#[test]
fn test_extension_parity_across_languages() {
    let rust = ExtensionCapabilities::new(LanguageRuntime::Rust);
    let python = ExtensionCapabilities::new(LanguageRuntime::Python);
    let js = ExtensionCapabilities::new(LanguageRuntime::JavaScript);
    // All runtimes use default capabilities - must be at full parity.
    assert!(rust.is_parity_with(&python));
    assert!(rust.is_parity_with(&js));
    assert!(python.is_parity_with(&js));
}

#[test]
fn test_parity_score_full() {
    let a = ExtensionCapabilities::new(LanguageRuntime::Rust);
    let b = ExtensionCapabilities::new(LanguageRuntime::Go);
    assert!((a.parity_score(&b) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_parity_score_partial() {
    let mut wasm = ExtensionCapabilities::new(LanguageRuntime::Wasm);
    wasm.supports_memory = false;
    let rust = ExtensionCapabilities::new(LanguageRuntime::Rust);
    let score = wasm.parity_score(&rust);
    // 2 out of 3 match.
    assert!((score - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn test_cross_runtime_parity_check() {
    let caps = vec![
        ExtensionCapabilities::new(LanguageRuntime::Rust),
        ExtensionCapabilities::new(LanguageRuntime::Python),
        ExtensionCapabilities::new(LanguageRuntime::Go),
    ];
    let pairs = check_cross_runtime_parity(&caps);
    // 3 runtimes -> 3 pairs.
    assert_eq!(pairs.len(), 3);
    for (_, _, score) in &pairs {
        assert!((score - 1.0).abs() < f64::EPSILON);
    }
}
