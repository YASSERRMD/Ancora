use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceKind {
    LogEntry,
    PolicyDocument,
    TestResult,
    Attestation,
    Screenshot,
    AuditRecord,
}

#[derive(Debug, Clone)]
pub struct EvidenceItem {
    pub id: String,
    pub kind: EvidenceKind,
    pub title: String,
    pub description: String,
    pub collected_tick: u64,
    pub tenant_id: String,
    pub metadata: HashMap<String, String>,
}

impl EvidenceItem {
    pub fn new(
        id: impl Into<String>,
        kind: EvidenceKind,
        title: impl Into<String>,
        description: impl Into<String>,
        collected_tick: u64,
        tenant_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            title: title.into(),
            description: description.into(),
            collected_tick,
            tenant_id: tenant_id.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

pub struct EvidenceStore {
    items: HashMap<String, EvidenceItem>,
}

impl EvidenceStore {
    pub fn new() -> Self { Self { items: HashMap::new() } }

    pub fn insert(&mut self, item: EvidenceItem) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn get(&self, id: &str) -> Option<&EvidenceItem> { self.items.get(id) }

    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&EvidenceItem> {
        self.items.values().filter(|e| e.tenant_id == tenant_id).collect()
    }

    pub fn count(&self) -> usize { self.items.len() }

    pub fn all(&self) -> impl Iterator<Item = &EvidenceItem> { self.items.values() }
}

impl Default for EvidenceStore {
    fn default() -> Self { Self::new() }
}
