use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a RAG-with-citations recipe from the given parameters.
pub fn build(params: &ParamSet) -> Recipe {
    let corpus = params.get("corpus").unwrap_or("documents");
    let top_k: usize = params
        .get("top_k")
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);

    let mut r = Recipe::new(
        "rag-citations",
        "RAG with Citations",
        "Retrieve relevant passages and generate an answer with inline citations.",
    );

    r.add_step(RecipeStep::new(
        "retrieve",
        StepAction::Retrieve,
        format!("Retrieve top {} passages from corpus '{}'", top_k, corpus),
    ));
    r.add_step(RecipeStep::new(
        "generate",
        StepAction::Generate,
        "Generate answer grounded in retrieved passages with [N] citations",
    ));
    r
}

/// A retrieved passage with a citation key.
#[derive(Debug, Clone)]
pub struct CitedPassage {
    pub key: usize,
    pub source: String,
    pub text: String,
}

/// Simulate retrieval from an in-memory corpus (no I/O).
pub fn retrieve_passages(corpus: &[(&str, &str)], _query: &str, top_k: usize) -> Vec<CitedPassage> {
    corpus
        .iter()
        .take(top_k)
        .enumerate()
        .map(|(i, (src, txt))| CitedPassage {
            key: i + 1,
            source: src.to_string(),
            text: txt.to_string(),
        })
        .collect()
}

/// Format passages into a citation block.
pub fn format_citations(passages: &[CitedPassage]) -> String {
    passages
        .iter()
        .map(|p| format!("[{}] {} - {}", p.key, p.source, p.text))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_has_two_steps() {
        let params = ParamSet::default();
        let r = build(&params);
        assert_eq!(r.step_count(), 2);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn retrieve_returns_top_k() {
        let corpus = vec![
            ("doc1", "passage one"),
            ("doc2", "passage two"),
            ("doc3", "passage three"),
        ];
        let result = retrieve_passages(&corpus, "query", 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].key, 1);
    }
}
