/// These tests verify that recipes run entirely offline - no network, no filesystem I/O.
/// All operations are in-memory and deterministic.
use crate::params::ParamSet;
use crate::{
    code_review, customer_support, data_extraction, debate, doc_processing, rag_citations,
    research_report,
};

#[test]
fn all_recipes_build_offline() {
    let ps = ParamSet::default();
    let recipes = vec![
        rag_citations::build(&ps),
        research_report::build(&ps),
        code_review::build(&ps),
        data_extraction::build(&ps),
        customer_support::build(&ps),
        debate::build(&ps),
        doc_processing::build(&ps),
    ];
    for r in &recipes {
        assert!(r.validate().is_ok(), "recipe '{}' failed validation", r.id);
    }
}

#[test]
fn recipe_step_counts_are_deterministic() {
    let ps = ParamSet::default();
    for _ in 0..3 {
        assert_eq!(rag_citations::build(&ps).step_count(), 2);
        assert_eq!(research_report::build(&ps).step_count(), 4);
        assert_eq!(code_review::build(&ps).step_count(), 4);
        assert_eq!(data_extraction::build(&ps).step_count(), 4);
        assert_eq!(customer_support::build(&ps).step_count(), 4);
        assert_eq!(debate::build(&ps).step_count(), 4);
        assert_eq!(doc_processing::build(&ps).step_count(), 4);
    }
}

#[test]
fn rag_retrieval_offline() {
    let corpus: Vec<(&str, &str)> = vec![("s1", "p1"), ("s2", "p2")];
    let result = rag_citations::retrieve_passages(&corpus, "q", 10);
    assert_eq!(result.len(), 2); // capped by corpus size
}

#[test]
fn doc_split_offline() {
    let text = "Para one.\n\nPara two.";
    let chunks = doc_processing::split_paragraphs(text);
    assert_eq!(chunks.len(), 2);
}
