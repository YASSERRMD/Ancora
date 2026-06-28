use crate::label::{DataCategory, SensitivityLevel};
use crate::record::DataRecord;

pub struct DataRecordBuilder {
    id: String,
    tenant_id: String,
    name: String,
    level: SensitivityLevel,
    category: DataCategory,
    created_tick: u64,
    tags: Vec<String>,
}

impl DataRecordBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            level: SensitivityLevel::Internal,
            category: DataCategory::Generic,
            created_tick: 0,
            tags: Vec::new(),
        }
    }

    pub fn level(mut self, level: SensitivityLevel) -> Self {
        self.level = level;
        self
    }

    pub fn category(mut self, category: DataCategory) -> Self {
        self.category = category;
        self
    }

    pub fn tick(mut self, tick: u64) -> Self {
        self.created_tick = tick;
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn build(self) -> DataRecord {
        let mut r = DataRecord::new(
            self.id, self.tenant_id, self.name, self.level, self.category, self.created_tick,
        );
        for t in self.tags { r = r.with_tag(t); }
        r
    }
}
