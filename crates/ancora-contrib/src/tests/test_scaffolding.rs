use crate::scaffolding::{scaffold, ScaffoldError, ScaffoldKind, ScaffoldRequest};

fn req(kind: ScaffoldKind, name: &str) -> ScaffoldRequest {
    ScaffoldRequest::new(kind, name)
}

#[test]
fn scaffold_provider_generates_files() {
    let output = scaffold(&req(ScaffoldKind::Provider, "AcmeCloud")).unwrap();
    let paths = output.paths();
    assert!(paths.contains(&"Cargo.toml"), "missing Cargo.toml");
    assert!(paths.contains(&"src/lib.rs"), "missing src/lib.rs");
    assert!(paths.contains(&"docs/README.md"), "missing docs/README.md");
}

#[test]
fn scaffold_tool_generates_conformance() {
    let output = scaffold(&req(ScaffoldKind::Tool, "WebSearch")).unwrap();
    let paths = output.paths();
    assert!(
        paths.contains(&"src/conformance.rs"),
        "missing conformance stub"
    );
}

#[test]
fn scaffold_generates_test_file() {
    let output = scaffold(&req(ScaffoldKind::Grader, "RougeL")).unwrap();
    let paths = output.paths();
    assert!(paths.contains(&"src/tests.rs"), "missing tests stub");
}

#[test]
fn scaffold_cargo_toml_has_crate_name() {
    let output = scaffold(&req(ScaffoldKind::VectorStore, "Pinecone")).unwrap();
    let cargo = output.get("Cargo.toml").expect("must have Cargo.toml");
    assert!(
        cargo.content.contains("ancora-pinecone"),
        "crate name not in Cargo.toml"
    );
}

#[test]
fn scaffold_lib_references_snake_name() {
    let output = scaffold(&req(ScaffoldKind::Provider, "AcmeCloud")).unwrap();
    let lib = output.get("src/lib.rs").expect("must have lib.rs");
    assert!(
        lib.content.contains("acme_cloud"),
        "snake_name not in lib.rs"
    );
}

#[test]
fn scaffold_invalid_name_with_space_returns_error() {
    let r = ScaffoldRequest::new(ScaffoldKind::Tool, "my tool");
    match scaffold(&r) {
        Err(ScaffoldError::InvalidName(_)) => {}
        other => panic!("expected InvalidName, got {other:?}"),
    }
}

#[test]
fn scaffold_empty_name_returns_error() {
    let r = ScaffoldRequest::new(ScaffoldKind::Tool, "");
    match scaffold(&r) {
        Err(ScaffoldError::InvalidName(_)) => {}
        other => panic!("expected InvalidName, got {other:?}"),
    }
}

#[test]
fn scaffold_kind_from_str_roundtrips() {
    for s in &[
        "provider",
        "tool",
        "grader",
        "guardrail",
        "exporter",
        "plugin",
    ] {
        let k = ScaffoldKind::from_str(s).unwrap_or_else(|_| panic!("parse failed for {s}"));
        assert_eq!(k.as_str(), *s, "as_str mismatch for {s}");
    }
}

#[test]
fn scaffold_kind_from_str_vector_store_aliases() {
    for alias in &["vectorstore", "vector_store", "vector-store"] {
        let k = ScaffoldKind::from_str(alias).unwrap();
        assert_eq!(k, ScaffoldKind::VectorStore);
    }
}

#[test]
fn scaffold_kind_from_str_unknown_returns_error() {
    match ScaffoldKind::from_str("frobnicator") {
        Err(ScaffoldError::UnknownKind(k)) => assert_eq!(k, "frobnicator"),
        other => panic!("expected UnknownKind, got {other:?}"),
    }
}

#[test]
fn scaffold_with_author_sets_author_in_cargo() {
    let r = ScaffoldRequest::new(ScaffoldKind::Tool, "MyTool").with_author("alice");
    let output = scaffold(&r).unwrap();
    let cargo = output.get("Cargo.toml").unwrap();
    assert!(cargo.content.contains("alice"));
}

#[test]
fn snake_name_conversion() {
    let r = ScaffoldRequest::new(ScaffoldKind::Tool, "AcmeCloudProvider");
    assert_eq!(r.snake_name(), "acme_cloud_provider");
}
