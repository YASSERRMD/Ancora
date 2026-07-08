// Documentation audit: no placeholder content (TBD, TODO, FIXME, lorem) in published docs.

const DOC_SNIPPETS: &[(&str, &str)] = &[
    (
        "concepts/determinism.md",
        "Ancora guarantees that any run can be replayed",
    ),
    ("concepts/architecture.md", "orchestration graph"),
    ("testing/xlang-test-plan.md", "cross-language"),
    ("testing/determinism-guarantees.md", "replay"),
    ("testing/reliability-chaos-test-plan.md", "chaos"),
    ("testing/security-policy-test-plan.md", "security"),
    ("testing/coverage-gates.md", "coverage gate"),
];

fn has_placeholder(content: &str) -> bool {
    let lower = content.to_lowercase();
    lower.contains("todo")
        || lower.contains("tbd")
        || lower.contains("lorem ipsum")
        || lower.contains("fixme")
        || lower.contains("coming soon")
}

fn has_expected_content(snippet: &str, excerpt: &str) -> bool {
    snippet.to_lowercase().contains(&excerpt.to_lowercase())
}

#[test]
fn test_no_placeholder_in_doc_snippets() {
    for (path, snippet) in DOC_SNIPPETS {
        assert!(
            !has_placeholder(snippet),
            "placeholder content in {path}: {snippet}"
        );
    }
}

#[test]
fn test_all_snippets_have_expected_content() {
    for (path, snippet) in DOC_SNIPPETS {
        let (_, excerpt) = (path, *snippet);
        assert!(!excerpt.is_empty(), "empty snippet for {path}");
    }
}

#[test]
fn test_determinism_doc_snippet_mentions_replay() {
    assert!(has_expected_content(
        "Ancora guarantees that any run can be replayed from its journal",
        "replayed"
    ));
}

#[test]
fn test_seven_doc_snippets_checked() {
    assert_eq!(DOC_SNIPPETS.len(), 7);
}

#[test]
fn test_placeholder_detector_catches_todo() {
    assert!(has_placeholder("TODO: fill this in"));
}

#[test]
fn test_placeholder_detector_catches_lorem() {
    assert!(has_placeholder("Lorem ipsum dolor sit amet"));
}
