use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootEvent {
    MeasurementAdded,
    AttestationReceived,
    PolicyChecked,
    SealOperation,
    UnsealOperation,
    ChainValidated,
}

impl fmt::Display for BootEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BootEvent::MeasurementAdded => "MEASUREMENT_ADDED",
            BootEvent::AttestationReceived => "ATTESTATION_RECEIVED",
            BootEvent::PolicyChecked => "POLICY_CHECKED",
            BootEvent::SealOperation => "SEAL_OPERATION",
            BootEvent::UnsealOperation => "UNSEAL_OPERATION",
            BootEvent::ChainValidated => "CHAIN_VALIDATED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct BootAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub node_id: String,
    pub event: BootEvent,
    pub subject: String,
    pub success: bool,
    pub detail: String,
}

impl BootAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        node_id: impl Into<String>,
        event: BootEvent,
        subject: impl Into<String>,
        success: bool,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            node_id: node_id.into(),
            event,
            subject: subject.into(),
            success,
            detail: detail.into(),
        }
    }
}

pub struct BootAuditLog {
    entries: VecDeque<BootAuditEntry>,
}

impl BootAuditLog {
    pub fn new() -> Self { Self { entries: VecDeque::new() } }
    pub fn record(&mut self, entry: BootAuditEntry) { self.entries.push_back(entry); }
    pub fn count(&self) -> usize { self.entries.len() }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a BootAuditEntry> {
        self.entries.iter().filter(|e| e.tenant_id == tenant_id).collect()
    }
    pub fn for_node<'a>(&'a self, node_id: &str) -> Vec<&'a BootAuditEntry> {
        self.entries.iter().filter(|e| e.node_id == node_id).collect()
    }
    pub fn failures<'a>(&'a self) -> Vec<&'a BootAuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &BootAuditEntry> { self.entries.iter() }
}
