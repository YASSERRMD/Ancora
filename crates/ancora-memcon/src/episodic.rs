/// An episodic memory entry observed across turns.
#[derive(Debug, Clone)]
pub struct EpisodicEntry {
    pub key: String,
    pub content: String,
    pub occurrences: u32,
}

/// A promoted semantic memory derived from recurring episodic facts.
#[derive(Debug, Clone)]
pub struct SemanticEntry {
    pub key: String,
    pub content: String,
}

/// Promotes frequently seen episodic entries to semantic memory.
pub struct EpisodicToSemanticPromoter {
    pub min_occurrences: u32,
}

impl EpisodicToSemanticPromoter {
    pub fn new(min_occurrences: u32) -> Self {
        Self { min_occurrences }
    }

    pub fn promote(&self, entries: &[EpisodicEntry]) -> Vec<SemanticEntry> {
        entries
            .iter()
            .filter(|e| e.occurrences >= self.min_occurrences)
            .map(|e| SemanticEntry { key: e.key.clone(), content: e.content.clone() })
            .collect()
    }
}
