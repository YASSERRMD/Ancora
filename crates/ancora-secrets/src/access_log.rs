use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessKind {
    Read,
    Write,
    Rotate,
    Delete,
}

#[derive(Debug, Clone)]
pub struct AccessRecord {
    pub tick: u64,
    pub tenant_id: String,
    pub path: String,
    pub subject: String,
    pub kind: AccessKind,
    pub version: Option<u32>,
}

impl AccessRecord {
    pub fn new(tick: u64, tenant_id: impl Into<String>, path: impl Into<String>, subject: impl Into<String>, kind: AccessKind) -> Self {
        Self { tick, tenant_id: tenant_id.into(), path: path.into(), subject: subject.into(), kind, version: None }
    }

    pub fn with_version(mut self, v: u32) -> Self { self.version = Some(v); self }
}

#[derive(Debug, Default)]
pub struct SecretAccessLog {
    records: VecDeque<AccessRecord>,
}

impl SecretAccessLog {
    pub fn new() -> Self { Self::default() }

    pub fn record(&mut self, entry: AccessRecord) { self.records.push_back(entry); }

    pub fn count(&self) -> usize { self.records.len() }

    pub fn reads_for(&self, tenant_id: &str, path: &str) -> Vec<&AccessRecord> {
        self.records.iter()
            .filter(|r| r.tenant_id == tenant_id && r.path == path && r.kind == AccessKind::Read)
            .collect()
    }

    pub fn all_for_tenant(&self, tenant_id: &str) -> Vec<&AccessRecord> {
        self.records.iter().filter(|r| r.tenant_id == tenant_id).collect()
    }

    pub fn all_for_path(&self, path: &str) -> Vec<&AccessRecord> {
        self.records.iter().filter(|r| r.path == path).collect()
    }
}
