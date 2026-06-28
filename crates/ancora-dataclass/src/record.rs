use std::collections::HashMap;
use crate::label::{DataCategory, SensitivityLevel};

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub level: SensitivityLevel,
    pub category: DataCategory,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_tick: u64,
}

impl DataRecord {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        name: impl Into<String>,
        level: SensitivityLevel,
        category: DataCategory,
        created_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            level,
            category,
            tags: Vec::new(),
            metadata: HashMap::new(),
            created_tick,
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn is_above_level(&self, threshold: &SensitivityLevel) -> bool {
        self.level.is_above(threshold)
    }

    pub fn is_at_least_level(&self, threshold: &SensitivityLevel) -> bool {
        self.level.is_at_least(threshold)
    }
}
