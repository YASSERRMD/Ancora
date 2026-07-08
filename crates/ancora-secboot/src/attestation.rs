use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttestationStatus {
    Trusted,
    Untrusted,
    Unknown,
}

impl fmt::Display for AttestationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AttestationStatus::Trusted => "TRUSTED",
            AttestationStatus::Untrusted => "UNTRUSTED",
            AttestationStatus::Unknown => "UNKNOWN",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct AttestationRecord {
    pub id: String,
    pub tenant_id: String,
    pub node_id: String,
    pub status: AttestationStatus,
    pub quote: String,
    pub tick: u64,
    pub metadata: HashMap<String, String>,
}

impl AttestationRecord {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        node_id: impl Into<String>,
        status: AttestationStatus,
        quote: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            node_id: node_id.into(),
            status,
            quote: quote.into(),
            tick,
            metadata: HashMap::new(),
        }
    }

    pub fn is_trusted(&self) -> bool {
        self.status == AttestationStatus::Trusted
    }
}

pub struct AttestationLog {
    entries: Vec<AttestationRecord>,
}

impl AttestationLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn record(&mut self, entry: AttestationRecord) {
        self.entries.push(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a AttestationRecord> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn for_node<'a>(&'a self, node_id: &str) -> Vec<&'a AttestationRecord> {
        self.entries
            .iter()
            .filter(|e| e.node_id == node_id)
            .collect()
    }
    pub fn trusted<'a>(&'a self) -> Vec<&'a AttestationRecord> {
        self.entries.iter().filter(|e| e.is_trusted()).collect()
    }
    pub fn untrusted<'a>(&'a self) -> Vec<&'a AttestationRecord> {
        self.entries
            .iter()
            .filter(|e| e.status == AttestationStatus::Untrusted)
            .collect()
    }
    pub fn all(&self) -> &[AttestationRecord] {
        &self.entries
    }
}
