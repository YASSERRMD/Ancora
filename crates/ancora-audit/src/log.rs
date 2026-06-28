use crate::entry::AuditEntry;
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct ImmutableAuditLog {
    entries: VecDeque<AuditEntry>,
    next_id: u64,
    max_size: Option<usize>,
}

impl ImmutableAuditLog {
    pub fn new() -> Self { Self { entries: VecDeque::new(), next_id: 1, max_size: None } }

    pub fn with_max_size(mut self, max: usize) -> Self { self.max_size = Some(max); self }

    pub fn append(&mut self, mut entry: AuditEntry) -> u64 {
        let id = self.next_id;
        entry.id = id;
        entry.checksum = entry.compute_checksum();
        self.next_id += 1;
        if let Some(max) = self.max_size {
            if self.entries.len() >= max { self.entries.pop_front(); }
        }
        self.entries.push_back(entry);
        id
    }

    pub fn get(&self, id: u64) -> Option<&AuditEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn count(&self) -> usize { self.entries.len() }

    pub fn entries(&self) -> impl Iterator<Item = &AuditEntry> { self.entries.iter() }

    pub fn verify_all(&self) -> bool { self.entries.iter().all(|e| e.verify()) }

    pub fn filter_by_tenant<'a>(&'a self, tenant_id: &'a str) -> Vec<&'a AuditEntry> {
        self.entries.iter().filter(|e| e.tenant_id == tenant_id).collect()
    }

    pub fn filter_by_subject<'a>(&'a self, subject: &'a str) -> Vec<&'a AuditEntry> {
        self.entries.iter().filter(|e| e.subject == subject).collect()
    }

    pub fn filter_by_operation<'a>(&'a self, op: &'a str) -> Vec<&'a AuditEntry> {
        self.entries.iter().filter(|e| e.operation == op).collect()
    }

    pub fn filter_by_tick_range(&self, from: u64, to: u64) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.tick >= from && e.tick <= to).collect()
    }
}
