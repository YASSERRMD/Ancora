use crate::embed::Embedding;

/// A stored item paired with its embedding vector.
#[derive(Debug, Clone)]
pub struct VectorEntry {
    pub text: String,
    pub embedding: Embedding,
}

/// Flat in-memory vector index using cosine similarity for recall.
pub struct VectorIndex {
    entries: Vec<VectorEntry>,
}

impl VectorIndex {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn insert(&mut self, entry: VectorEntry) {
        self.entries.push(entry);
    }
}

impl Default for VectorIndex {
    fn default() -> Self {
        Self::new()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 { 0.0 } else { dot / (mag_a * mag_b) }
}

impl VectorIndex {
    /// Return the top-`k` entries most similar to `query` by cosine similarity.
    pub fn search(&self, query: &[f32], k: usize) -> Vec<&VectorEntry> {
        let mut scored: Vec<(f32, usize)> = self.entries.iter().enumerate()
            .map(|(i, e)| (cosine_similarity(query, &e.embedding), i))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(k).map(|(_, i)| &self.entries[i]).collect()
    }
}
