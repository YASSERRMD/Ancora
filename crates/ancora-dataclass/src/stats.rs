use crate::label::SensitivityLevel;
use crate::record::DataRecord;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataClassStats {
    pub total: usize,
    pub by_level: HashMap<String, usize>,
    pub highest_level: Option<SensitivityLevel>,
}

impl DataClassStats {
    pub fn from_records<'a>(records: impl Iterator<Item = &'a DataRecord>) -> Self {
        let mut total = 0usize;
        let mut by_level: HashMap<String, usize> = HashMap::new();
        let mut highest: Option<SensitivityLevel> = None;
        for r in records {
            total += 1;
            *by_level.entry(format!("{}", r.level)).or_insert(0) += 1;
            if highest.as_ref().map_or(true, |h| r.level.is_above(h)) {
                highest = Some(r.level.clone());
            }
        }
        Self {
            total,
            by_level,
            highest_level: highest,
        }
    }

    pub fn count_at_level(&self, level: &SensitivityLevel) -> usize {
        *self.by_level.get(&format!("{level}")).unwrap_or(&0)
    }
}
