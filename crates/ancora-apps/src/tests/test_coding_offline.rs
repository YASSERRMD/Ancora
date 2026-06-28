use crate::coding_assistant::{CodingAssistant, Language, Snippet, SnippetLibrary};

#[test]
fn coding_assistant_finds_snippet_offline() {
    let mut lib = SnippetLibrary::new();
    lib.add(Snippet::new(
        "rust-error-handling",
        Language::Rust,
        "propagate errors with the question mark operator",
        "fn read_config(path: &str) -> Result<String, std::io::Error> { std::fs::read_to_string(path) }",
    ));
    lib.add(Snippet::new(
        "python-list-comp",
        Language::Python,
        "list comprehension with filter",
        "squares = [x**2 for x in range(10) if x % 2 == 0]",
    ));

    let assistant = CodingAssistant::new(lib);

    let suggestion = assistant.suggest("errors", Some(&Language::Rust));
    assert!(!suggestion.snippets.is_empty());
    assert!(suggestion.snippets[0].contains("Result"));
}

#[test]
fn coding_assistant_cross_language_search() {
    let mut lib = SnippetLibrary::new();
    lib.add(Snippet::new("r1", Language::Rust, "write to file", "std::fs::write(path, data)?;"));
    lib.add(Snippet::new("p1", Language::Python, "write to file", "open(path, 'w').write(data)"));

    let assistant = CodingAssistant::new(lib);
    let suggestion = assistant.suggest("write to file", None);
    assert_eq!(suggestion.snippets.len(), 2);
}

#[test]
fn struct_stub_contains_name() {
    let assistant = CodingAssistant::new(SnippetLibrary::new());
    let stub = assistant.generate_struct_stub("AgentConfig");
    assert!(stub.contains("AgentConfig"));
    assert!(stub.contains("pub struct"));
}
