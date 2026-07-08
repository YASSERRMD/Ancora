use crate::entry::MemoryEntry;

/// Keyword-based retrieval for offline use (no embeddings).
pub struct KeywordRetriever;

impl KeywordRetriever {
    pub fn retrieve<'a>(
        memories: &[&'a MemoryEntry],
        query: &str,
        top_k: usize,
    ) -> Vec<&'a MemoryEntry> {
        let query_lower = query.to_lowercase();
        let query_words: Vec<&str> = query_lower.split_whitespace().collect();

        let mut scored: Vec<(&MemoryEntry, usize)> = memories
            .iter()
            .map(|e| {
                let content_lower = e.content.to_lowercase();
                let matches = query_words
                    .iter()
                    .filter(|w| content_lower.contains(*w))
                    .count();
                (*e, matches)
            })
            .filter(|(_, score)| *score > 0)
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.iter().take(top_k).map(|(e, _)| *e).collect()
    }
}
