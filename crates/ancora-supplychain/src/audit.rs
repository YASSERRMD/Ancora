use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupplyChainEvent {
    ComponentAdded,
    ComponentSigned,
    ComponentVerified,
    ProvenanceRecorded,
    SbomGenerated,
    PolicyChecked,
}

impl fmt::Display for SupplyChainEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SupplyChainEvent::ComponentAdded => "COMPONENT_ADDED",
            SupplyChainEvent::ComponentSigned => "COMPONENT_SIGNED",
            SupplyChainEvent::ComponentVerified => "COMPONENT_VERIFIED",
            SupplyChainEvent::ProvenanceRecorded => "PROVENANCE_RECORDED",
            SupplyChainEvent::SbomGenerated => "SBOM_GENERATED",
            SupplyChainEvent::PolicyChecked => "POLICY_CHECKED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct SupplyChainAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub component_id: String,
    pub event: SupplyChainEvent,
    pub subject: String,
    pub success: bool,
}

impl SupplyChainAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        component_id: impl Into<String>,
        event: SupplyChainEvent,
        subject: impl Into<String>,
        success: bool,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            component_id: component_id.into(),
            event,
            subject: subject.into(),
            success,
        }
    }
}

pub struct SupplyChainAuditLog {
    entries: VecDeque<SupplyChainAuditEntry>,
}

impl SupplyChainAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: SupplyChainAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a SupplyChainAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn for_component<'a>(&'a self, component_id: &str) -> Vec<&'a SupplyChainAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.component_id == component_id)
            .collect()
    }
    pub fn failures<'a>(&'a self) -> Vec<&'a SupplyChainAuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &SupplyChainAuditEntry> {
        self.entries.iter()
    }
}
