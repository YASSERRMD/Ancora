use crate::polyglot::{full_router, Language, PolyglotRouter, RouterError};

#[test]
fn full_router_has_all_languages() {
    let router = full_router();
    let langs = router.registered_languages();
    assert_eq!(langs.len(), 6);
}

#[test]
fn dispatch_go_to_python() {
    let router = full_router();
    let msg = router
        .dispatch(&Language::Go, &Language::Python, "ping")
        .unwrap();
    assert_eq!(msg.from_language, "go");
    assert_eq!(msg.to_language, "python");
    assert_eq!(msg.payload, "ping");
    assert!(msg.trace_id.starts_with("a2a-go-python-"));
}

#[test]
fn dispatch_rust_to_java() {
    let router = full_router();
    let msg = router
        .dispatch(&Language::Rust, &Language::Java, "hello")
        .unwrap();
    assert_eq!(msg.from_language, "rust");
    assert_eq!(msg.to_language, "java");
}

#[test]
fn dispatch_rejects_empty_payload() {
    let router = full_router();
    let err = router
        .dispatch(&Language::Go, &Language::Python, "")
        .unwrap_err();
    assert_eq!(err, RouterError::EmptyPayload);
}

#[test]
fn dispatch_rejects_unknown_target() {
    let mut router = PolyglotRouter::new();
    router.register(Language::Go, "in-process://go");
    let err = router
        .dispatch(&Language::Go, &Language::Python, "msg")
        .unwrap_err();
    assert_eq!(
        err,
        RouterError::NoEndpointForLanguage("python".to_string())
    );
}

#[test]
fn all_language_pairs_can_dispatch() {
    let router = full_router();
    let langs = [
        Language::Go,
        Language::Python,
        Language::TypeScript,
        Language::DotNet,
        Language::Java,
        Language::Rust,
    ];
    for from in &langs {
        for to in &langs {
            let result = router.dispatch(from, to, "test");
            assert!(result.is_ok(), "dispatch {:?} -> {:?} failed", from, to);
        }
    }
}
