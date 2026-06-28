// Documentation audit: key concepts cross-reference each other.

struct ConceptLink {
    source: &'static str,
    links_to: &'static str,
}

const CROSS_REFS: &[ConceptLink] = &[
    ConceptLink { source: "determinism",           links_to: "durability-and-replay" },
    ConceptLink { source: "durability-and-replay", links_to: "human-in-the-loop" },
    ConceptLink { source: "agents",                links_to: "orchestration-graph" },
    ConceptLink { source: "orchestration-graph",   links_to: "tools-and-effects" },
    ConceptLink { source: "memory-tiers",          links_to: "vector-stores" },
    ConceptLink { source: "observability-and-otel",links_to: "determinism" },
    ConceptLink { source: "policy",                links_to: "providers-and-local-first" },
    ConceptLink { source: "mcp-and-a2a",           links_to: "agents" },
];

fn has_cross_ref(source: &str, target: &str) -> bool {
    CROSS_REFS.iter().any(|r| r.source == source && r.links_to == target)
}

#[test]
fn test_8_cross_refs_defined() {
    assert_eq!(CROSS_REFS.len(), 8);
}

#[test]
fn test_determinism_links_to_replay() {
    assert!(has_cross_ref("determinism", "durability-and-replay"));
}

#[test]
fn test_agents_links_to_orchestration_graph() {
    assert!(has_cross_ref("agents", "orchestration-graph"));
}

#[test]
fn test_memory_tiers_links_to_vector_stores() {
    assert!(has_cross_ref("memory-tiers", "vector-stores"));
}

#[test]
fn test_no_cross_ref_links_to_itself() {
    for r in CROSS_REFS { assert_ne!(r.source, r.links_to, "{} links to itself", r.source); }
}

#[test]
fn test_all_link_targets_are_concept_slugs() {
    let targets: Vec<&str> = CROSS_REFS.iter().map(|r| r.links_to).collect();
    for t in targets { assert!(!t.contains('/'), "target should be a slug, not a path: {t}"); }
}
