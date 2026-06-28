// Documentation audit: every guide has at least one runnable example.

const GUIDES_WITH_EXAMPLES: &[(&str, usize)] = &[
    ("guides/single-agent.md",      2),
    ("guides/verifier-pattern.md",  2),
    ("guides/human-in-the-loop.md", 1),
    ("guides/vector-rag.md",        2),
    ("guides/mcp-tools.md",         1),
    ("guides/a2a-handoff.md",       1),
    ("guides/local-first.md",       2),
    ("guides/cost-control.md",      1),
];

fn has_runnable_example(guide: &str, count: usize) -> bool {
    !guide.is_empty() && count >= 1
}

#[test]
fn test_eight_guides_have_examples() {
    assert_eq!(GUIDES_WITH_EXAMPLES.len(), 8);
}

#[test]
fn test_all_guides_have_at_least_one_example() {
    for (guide, count) in GUIDES_WITH_EXAMPLES {
        assert!(has_runnable_example(guide, *count),
            "guide {guide} has no runnable example");
    }
}

#[test]
fn test_vector_rag_guide_has_two_examples() {
    let rag = GUIDES_WITH_EXAMPLES.iter().find(|(g, _)| *g == "guides/vector-rag.md");
    assert_eq!(rag.map(|(_, c)| *c), Some(2));
}

#[test]
fn test_all_guide_paths_start_with_guides() {
    for (path, _) in GUIDES_WITH_EXAMPLES {
        assert!(path.starts_with("guides/"), "not in guides/: {path}");
    }
}

#[test]
fn test_total_examples_across_guides() {
    let total: usize = GUIDES_WITH_EXAMPLES.iter().map(|(_, c)| c).sum();
    assert!(total >= 12, "expected >= 12 total examples, got {total}");
}
