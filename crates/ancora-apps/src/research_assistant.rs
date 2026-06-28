/// Research assistant application.
///
/// Aggregates knowledge from a local knowledge base and synthesises
/// summaries without network access.

#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub topic: String,
    pub body: String,
    pub tags: Vec<String>,
}

impl KnowledgeEntry {
    pub fn new(topic: impl Into<String>, body: impl Into<String>, tags: Vec<String>) -> Self {
        Self {
            topic: topic.into(),
            body: body.into(),
            tags,
        }
    }
}

#[derive(Debug, Default)]
pub struct KnowledgeBase {
    entries: Vec<KnowledgeEntry>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: KnowledgeEntry) {
        self.entries.push(entry);
    }

    pub fn find_by_tag(&self, tag: &str) -> Vec<&KnowledgeEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect()
    }

    pub fn find_by_topic(&self, topic: &str) -> Option<&KnowledgeEntry> {
        self.entries
            .iter()
            .find(|e| e.topic.eq_ignore_ascii_case(topic))
    }
}

#[derive(Debug, Clone)]
pub struct ResearchSummary {
    pub query: String,
    pub bullets: Vec<String>,
}

pub struct ResearchAssistant {
    kb: KnowledgeBase,
}

impl ResearchAssistant {
    pub fn new(kb: KnowledgeBase) -> Self {
        Self { kb }
    }

    /// Synthesise a summary for the given topic or tag.
    pub fn research(&self, query: &str) -> ResearchSummary {
        let mut bullets: Vec<String> = Vec::new();

        // Try exact topic match first.
        if let Some(entry) = self.kb.find_by_topic(query) {
            bullets.push(entry.body.clone());
        }

        // Also include tag matches.
        for entry in self.kb.find_by_tag(query) {
            let bullet = format!("[{}] {}", entry.topic, entry.body);
            if !bullets.contains(&bullet) {
                bullets.push(bullet);
            }
        }

        ResearchSummary {
            query: query.to_string(),
            bullets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_returns_bullets() {
        let mut kb = KnowledgeBase::new();
        kb.add(KnowledgeEntry::new(
            "Rust",
            "Rust is a systems programming language focused on safety.",
            vec!["systems".to_string(), "language".to_string()],
        ));
        let ra = ResearchAssistant::new(kb);
        let summary = ra.research("Rust");
        assert!(!summary.bullets.is_empty());
    }
}
