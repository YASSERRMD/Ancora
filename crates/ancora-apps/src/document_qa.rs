/// Document question-answering application.
///
/// Provides offline document ingestion and query answering over a local
/// corpus without any network calls.

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl Document {
    pub fn new(id: impl Into<String>, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            content: content.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentStore {
    documents: Vec<Document>,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self { documents: Vec::new() }
    }

    pub fn ingest(&mut self, doc: Document) {
        self.documents.push(doc);
    }

    pub fn len(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Return document IDs whose content contains the query string (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<&Document> {
        let q = query.to_lowercase();
        self.documents
            .iter()
            .filter(|d| d.content.to_lowercase().contains(&q) || d.title.to_lowercase().contains(&q))
            .collect()
    }
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub query: String,
    pub excerpts: Vec<String>,
    pub source_ids: Vec<String>,
}

/// A simple offline QA engine that extracts relevant excerpts.
pub struct DocumentQaEngine {
    store: DocumentStore,
}

impl DocumentQaEngine {
    pub fn new(store: DocumentStore) -> Self {
        Self { store }
    }

    pub fn ask(&self, query: &str) -> Answer {
        let hits = self.store.search(query);
        let mut excerpts = Vec::new();
        let mut source_ids = Vec::new();

        for doc in hits {
            // Extract the first sentence that contains the query term.
            let q_lower = query.to_lowercase();
            for sentence in doc.content.split('.') {
                if sentence.to_lowercase().contains(&q_lower) {
                    excerpts.push(sentence.trim().to_string());
                    break;
                }
            }
            source_ids.push(doc.id.clone());
        }

        Answer {
            query: query.to_string(),
            excerpts,
            source_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_contains_source_id() {
        let mut store = DocumentStore::new();
        store.ingest(Document::new("doc1", "Policy", "The retention policy requires 7 years."));
        let engine = DocumentQaEngine::new(store);
        let answer = engine.ask("retention");
        assert!(answer.source_ids.contains(&"doc1".to_string()));
        assert!(!answer.excerpts.is_empty());
    }
}
