use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a document-processing recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let doc_type = params.get("doc_type").unwrap_or("generic");
    let chunking = params.get("chunking").unwrap_or("paragraph");

    let mut r = Recipe::new(
        "document-processing",
        "Document Processing",
        format!(
            "Ingest, chunk (by {}), and process {} documents.",
            chunking, doc_type
        ),
    );

    r.add_step(RecipeStep::new(
        "ingest",
        StepAction::Extract,
        format!("Load and parse {} document", doc_type),
    ));
    r.add_step(RecipeStep::new(
        "chunk",
        StepAction::Extract,
        format!("Split document into chunks by {}", chunking),
    ));
    r.add_step(RecipeStep::new(
        "enrich",
        StepAction::Generate,
        "Add metadata tags and summaries to each chunk",
    ));
    r.add_step(RecipeStep::new(
        "index",
        StepAction::Custom("index".to_string()),
        "Store enriched chunks in the document index",
    ));
    r
}

/// Chunking strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkStrategy {
    Paragraph,
    Sentence,
    FixedTokens(usize),
}

/// A document chunk.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub index: usize,
    pub text: String,
    pub metadata: Vec<(String, String)>,
}

impl Chunk {
    pub fn new(index: usize, text: impl Into<String>) -> Self {
        Self {
            index,
            text: text.into(),
            metadata: Vec::new(),
        }
    }

    pub fn add_meta(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.push((key.into(), value.into()));
    }

    pub fn get_meta(&self, key: &str) -> Option<&str> {
        self.metadata
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// Split text into paragraph chunks (simple offline split).
pub fn split_paragraphs(text: &str) -> Vec<Chunk> {
    text.split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .enumerate()
        .map(|(i, s)| Chunk::new(i, s.trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_has_four_steps() {
        let params = ParamSet::default();
        let r = build(&params);
        assert_eq!(r.step_count(), 4);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn paragraph_split() {
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird.";
        let chunks = split_paragraphs(text);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "First paragraph.");
    }

    #[test]
    fn chunk_metadata() {
        let mut chunk = Chunk::new(0, "hello");
        chunk.add_meta("lang", "en");
        assert_eq!(chunk.get_meta("lang"), Some("en"));
        assert_eq!(chunk.get_meta("missing"), None);
    }
}
