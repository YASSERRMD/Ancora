use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryKind {
    Fact,
    Preference,
    Instruction,
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub kind: MemoryKind,
    pub content: String,
    pub importance: u8,  // 1-10
    pub created_at: u64,
    pub last_accessed: u64,
    pub access_count: u32,
}

impl MemoryEntry {
    pub fn new(id: &str, kind: MemoryKind, content: &str, importance: u8, now: u64) -> Self {
        Self {
            id: id.to_string(),
            kind,
            content: content.to_string(),
            importance: importance.clamp(1, 10),
            created_at: now,
            last_accessed: now,
            access_count: 0,
        }
    }

    pub fn access(&mut self, now: u64) {
        self.last_accessed = now;
        self.access_count += 1;
    }

    pub fn score(&self, now: u64) -> f64 {
        let recency = 1.0 / (1.0 + (now.saturating_sub(self.last_accessed) as f64 / 3600.0));
        let freq = (self.access_count as f64).ln_1p();
        self.importance as f64 * recency * (1.0 + freq)
    }
}
