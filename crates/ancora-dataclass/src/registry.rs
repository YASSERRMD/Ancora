use crate::label::SensitivityLevel;
use crate::record::DataRecord;
use std::collections::HashMap;

#[derive(Debug)]
pub enum RegistryError {
    RecordNotFound(String),
    RecordAlreadyExists(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::RecordNotFound(id) => write!(f, "record not found: {id}"),
            RegistryError::RecordAlreadyExists(id) => write!(f, "record already exists: {id}"),
        }
    }
}

pub struct DataRegistry {
    records: HashMap<String, DataRecord>,
}

impl DataRegistry {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn insert(&mut self, record: DataRecord) -> Result<(), RegistryError> {
        if self.records.contains_key(&record.id) {
            return Err(RegistryError::RecordAlreadyExists(record.id.clone()));
        }
        self.records.insert(record.id.clone(), record);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<&DataRecord, RegistryError> {
        self.records
            .get(id)
            .ok_or_else(|| RegistryError::RecordNotFound(id.to_string()))
    }

    pub fn remove(&mut self, id: &str) -> Result<DataRecord, RegistryError> {
        self.records
            .remove(id)
            .ok_or_else(|| RegistryError::RecordNotFound(id.to_string()))
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn by_tenant(&self, tenant_id: &str) -> Vec<&DataRecord> {
        self.records
            .values()
            .filter(|r| r.tenant_id == tenant_id)
            .collect()
    }

    pub fn at_or_above(&self, level: &SensitivityLevel) -> Vec<&DataRecord> {
        self.records
            .values()
            .filter(|r| r.level.is_at_least(level))
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &DataRecord> {
        self.records.values()
    }
}

impl Default for DataRegistry {
    fn default() -> Self {
        Self::new()
    }
}
