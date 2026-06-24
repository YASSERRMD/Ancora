use crate::tier::MemoryTier;

/// A single item stored in memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub tier: MemoryTier,
}

impl MemoryEntry {
    pub fn new(key: impl Into<String>, value: impl Into<String>, tier: MemoryTier) -> Self {
        Self { key: key.into(), value: value.into(), tier }
    }
}
