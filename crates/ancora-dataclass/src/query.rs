use crate::label::{DataCategory, SensitivityLevel};
use crate::record::DataRecord;

#[derive(Default)]
pub struct DataQuery {
    pub level: Option<SensitivityLevel>,
    pub min_level: Option<SensitivityLevel>,
    pub category: Option<DataCategory>,
    pub tag: Option<String>,
    pub tenant_id: Option<String>,
}

impl DataQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: SensitivityLevel) -> Self {
        self.level = Some(level);
        self
    }
    pub fn min_level(mut self, level: SensitivityLevel) -> Self {
        self.min_level = Some(level);
        self
    }
    pub fn category(mut self, cat: DataCategory) -> Self {
        self.category = Some(cat);
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn run<'a>(&self, records: impl Iterator<Item = &'a DataRecord>) -> Vec<&'a DataRecord> {
        records
            .filter(|r| {
                if let Some(ref l) = self.level {
                    if &r.level != l {
                        return false;
                    }
                }
                if let Some(ref ml) = self.min_level {
                    if !r.level.is_at_least(ml) {
                        return false;
                    }
                }
                if let Some(ref c) = self.category {
                    if &r.category != c {
                        return false;
                    }
                }
                if let Some(ref t) = self.tag {
                    if !r.has_tag(t) {
                        return false;
                    }
                }
                if let Some(ref tid) = self.tenant_id {
                    if &r.tenant_id != tid {
                        return false;
                    }
                }
                true
            })
            .collect()
    }
}
