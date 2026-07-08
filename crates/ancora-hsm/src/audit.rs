use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmOperation {
    GenerateKey,
    DeleteKey,
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    WrapKey,
    UnwrapKey,
    SessionOpened,
    SessionClosed,
}

impl fmt::Display for HsmOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HsmOperation::GenerateKey => "GENERATE_KEY",
            HsmOperation::DeleteKey => "DELETE_KEY",
            HsmOperation::Sign => "SIGN",
            HsmOperation::Verify => "VERIFY",
            HsmOperation::Encrypt => "ENCRYPT",
            HsmOperation::Decrypt => "DECRYPT",
            HsmOperation::WrapKey => "WRAP_KEY",
            HsmOperation::UnwrapKey => "UNWRAP_KEY",
            HsmOperation::SessionOpened => "SESSION_OPENED",
            HsmOperation::SessionClosed => "SESSION_CLOSED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct HsmAuditEntry {
    pub tick: u64,
    pub slot_id: u32,
    pub session_id: Option<u64>,
    pub key_handle: Option<u64>,
    pub operation: HsmOperation,
    pub success: bool,
    pub detail: String,
}

impl HsmAuditEntry {
    pub fn new(
        tick: u64,
        slot_id: u32,
        operation: HsmOperation,
        success: bool,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            slot_id,
            session_id: None,
            key_handle: None,
            operation,
            success,
            detail: detail.into(),
        }
    }
    pub fn with_session(mut self, id: u64) -> Self {
        self.session_id = Some(id);
        self
    }
    pub fn with_key(mut self, handle: u64) -> Self {
        self.key_handle = Some(handle);
        self
    }
}

pub struct HsmAuditLog {
    entries: VecDeque<HsmAuditEntry>,
}

impl Default for HsmAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: HsmAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_slot(&self, slot_id: u32) -> Vec<&HsmAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.slot_id == slot_id)
            .collect()
    }
    pub fn by_operation<'a>(&'a self, op: &HsmOperation) -> Vec<&'a HsmAuditEntry> {
        self.entries.iter().filter(|e| &e.operation == op).collect()
    }
    pub fn failures(&self) -> Vec<&HsmAuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &HsmAuditEntry> {
        self.entries.iter()
    }
}
