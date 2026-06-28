use crate::params::ParamSet;
use crate::rag_citations::{build, format_citations, retrieve_passages};

#[test]
fn rag_recipe_builds_and_validates() {
    let mut ps = ParamSet::new();
    ps.set("corpus", "my-docs");
    ps.set("top_k", "3");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "rag-citations");
}

#[test]
fn rag_retrieval_returns_correct_count() {
    let corpus = vec![
        ("src/a.md", "Alpha text"),
        ("src/b.md", "Beta text"),
        ("src/c.md", "Gamma text"),
        ("src/d.md", "Delta text"),
    ];
    let passages = retrieve_passages(&corpus, "what is alpha?", 2);
    assert_eq!(passages.len(), 2);
    assert_eq!(passages[0].source, "src/a.md");
}

#[test]
fn rag_citation_format_is_nonempty() {
    let corpus = vec![("doc1", "passage one")];
    let passages = retrieve_passages(&corpus, "query", 5);
    let formatted = format_citations(&passages);
    assert!(formatted.contains("[1]"));
    assert!(formatted.contains("doc1"));
}

#[test]
fn rag_default_params_work() {
    let ps = ParamSet::default();
    let r = build(&ps);
    assert_eq!(r.step_count(), 2);
}
