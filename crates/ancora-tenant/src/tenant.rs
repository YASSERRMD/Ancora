use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantStatus {
    Active,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub status: TenantStatus,
    pub created_tick: u64,
    pub metadata: HashMap<String, String>,
}

impl Tenant {
    pub fn new(id: impl Into<String>, name: impl Into<String>, created_tick: u64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            status: TenantStatus::Active,
            created_tick,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn is_active(&self) -> bool { self.status == TenantStatus::Active }
    pub fn is_suspended(&self) -> bool { self.status == TenantStatus::Suspended }
    pub fn is_deleted(&self) -> bool { self.status == TenantStatus::Deleted }

    pub fn suspend(&mut self) { self.status = TenantStatus::Suspended; }
    pub fn activate(&mut self) { self.status = TenantStatus::Active; }
    pub fn delete(&mut self) { self.status = TenantStatus::Deleted; }
}
