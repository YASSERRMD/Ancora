use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyOperation {
    Create,
    Read,
    Rotate,
    Deactivate,
    Destroy,
    Compromise,
}

impl std::fmt::Display for KeyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyOperation::Create => write!(f, "CREATE"),
            KeyOperation::Read => write!(f, "READ"),
            KeyOperation::Rotate => write!(f, "ROTATE"),
            KeyOperation::Deactivate => write!(f, "DEACTIVATE"),
            KeyOperation::Destroy => write!(f, "DESTROY"),
            KeyOperation::Compromise => write!(f, "COMPROMISE"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub key_id: String,
    pub version: u32,
    pub operation: KeyOperation,
    pub subject: String,
    pub success: bool,
}

impl KeyAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        key_id: impl Into<String>,
        version: u32,
        operation: KeyOperation,
        subject: impl Into<String>,
        success: bool,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            key_id: key_id.into(),
            version,
            operation,
            subject: subject.into(),
            success,
        }
    }
}

#[derive(Debug, Default)]
pub struct KeyAuditLog {
    entries: VecDeque<KeyAuditEntry>,
}

impl KeyAuditLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, entry: KeyAuditEntry) {
        self.entries.push_back(entry);
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn for_key(&self, key_id: &str) -> Vec<&KeyAuditEntry> {
        self.entries.iter().filter(|e| e.key_id == key_id).collect()
    }

    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&KeyAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }

    pub fn rotations_for(&self, key_id: &str) -> Vec<&KeyAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.key_id == key_id && e.operation == KeyOperation::Rotate)
            .collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &KeyAuditEntry> {
        self.entries.iter()
    }
}
