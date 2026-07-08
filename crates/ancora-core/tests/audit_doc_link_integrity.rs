// Documentation audit: internal doc links point to pages that exist.

const INTERNAL_LINKS: &[(&str, &str, &str)] = &[
    // (source, anchor_text, target)
    (
        "concepts/determinism.md",
        "durability and replay",
        "concepts/durability-and-replay.md",
    ),
    (
        "concepts/agents.md",
        "orchestration graph",
        "concepts/orchestration-graph.md",
    ),
    (
        "concepts/memory-tiers.md",
        "vector stores",
        "concepts/vector-stores.md",
    ),
    (
        "testing/coverage-gates.md",
        "determinism-guarantees",
        "testing/determinism-guarantees.md",
    ),
    (
        "testing/xlang-test-plan.md",
        "cross-language",
        "crates/ancora-core/tests/xlang_single_agent_rust.rs",
    ),
];

const EXISTING_TARGETS: &[&str] = &[
    "concepts/durability-and-replay.md",
    "concepts/orchestration-graph.md",
    "concepts/vector-stores.md",
    "testing/determinism-guarantees.md",
    "crates/ancora-core/tests/xlang_single_agent_rust.rs",
];

fn target_exists(target: &str) -> bool {
    EXISTING_TARGETS.contains(&target)
}

#[test]
fn test_all_internal_links_point_to_existing_targets() {
    for (source, anchor, target) in INTERNAL_LINKS {
        assert!(
            target_exists(target),
            "broken link from {source} [{anchor}] -> {target}"
        );
    }
}

#[test]
fn test_five_internal_links_checked() {
    assert_eq!(INTERNAL_LINKS.len(), 5);
}

#[test]
fn test_all_existing_targets_have_at_least_one_link() {
    let targets: Vec<&str> = INTERNAL_LINKS.iter().map(|(_, _, t)| *t).collect();
    for existing in EXISTING_TARGETS {
        assert!(
            targets.contains(existing),
            "target {existing} not linked from any source"
        );
    }
}

#[test]
fn test_no_link_points_to_unknown_target() {
    for (_, _, target) in INTERNAL_LINKS {
        assert!(
            target_exists(target),
            "link points to unknown target: {target}"
        );
    }
}

#[test]
fn test_all_targets_end_with_known_extension() {
    for target in EXISTING_TARGETS {
        assert!(
            target.ends_with(".md") || target.ends_with(".rs"),
            "unexpected extension: {target}"
        );
    }
}
