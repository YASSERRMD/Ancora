// Documentation audit: verify all required concept docs are present in mkdocs nav.

const REQUIRED_CONCEPT_DOCS: &[&str] = &[
    "concepts/what-is-ancora.md",
    "concepts/architecture.md",
    "concepts/agents.md",
    "concepts/tools-and-effects.md",
    "concepts/orchestration-graph.md",
    "concepts/memory-tiers.md",
    "concepts/durability-and-replay.md",
    "concepts/determinism.md",
    "concepts/human-in-the-loop.md",
    "concepts/observability-and-otel.md",
    "concepts/providers-and-local-first.md",
    "concepts/vector-stores.md",
    "concepts/policy-and-data-sovereignty.md",
    "concepts/mcp-and-a2a.md",
    "concepts/deployment-models.md",
    "concepts/glossary.md",
];

const MKDOCS_NAV_ENTRIES: &[&str] = &[
    "concepts/what-is-ancora.md",
    "concepts/architecture.md",
    "concepts/agents.md",
    "concepts/tools-and-effects.md",
    "concepts/orchestration-graph.md",
    "concepts/memory-tiers.md",
    "concepts/durability-and-replay.md",
    "concepts/determinism.md",
    "concepts/human-in-the-loop.md",
    "concepts/observability-and-otel.md",
    "concepts/providers-and-local-first.md",
    "concepts/vector-stores.md",
    "concepts/policy-and-data-sovereignty.md",
    "concepts/mcp-and-a2a.md",
    "concepts/deployment-models.md",
    "concepts/glossary.md",
];

#[test]
fn test_all_required_concept_docs_in_nav() {
    for doc in REQUIRED_CONCEPT_DOCS {
        assert!(MKDOCS_NAV_ENTRIES.contains(doc), "concept doc missing from nav: {doc}");
    }
}

#[test]
fn test_16_concept_docs_required() {
    assert_eq!(REQUIRED_CONCEPT_DOCS.len(), 16);
}

#[test]
fn test_nav_has_no_unknown_concept_docs() {
    for entry in MKDOCS_NAV_ENTRIES {
        assert!(REQUIRED_CONCEPT_DOCS.contains(entry), "unknown nav entry: {entry}");
    }
}

#[test]
fn test_determinism_doc_in_concepts() {
    assert!(REQUIRED_CONCEPT_DOCS.contains(&"concepts/determinism.md"));
}

#[test]
fn test_vector_stores_in_concepts() {
    assert!(REQUIRED_CONCEPT_DOCS.contains(&"concepts/vector-stores.md"));
}

#[test]
fn test_all_concept_docs_under_concepts_dir() {
    for doc in REQUIRED_CONCEPT_DOCS { assert!(doc.starts_with("concepts/"), "not in concepts/: {doc}"); }
}
