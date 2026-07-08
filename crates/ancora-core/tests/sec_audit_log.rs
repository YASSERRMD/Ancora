// Security: audit log -- every sensitive operation produces an immutable audit entry.

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct AuditEntry {
    operation: &'static str,
    actor: String,
    resource: String,
    allowed: bool,
    seq: u64,
}

struct AuditLog {
    entries: Vec<AuditEntry>,
    seq: u64,
}

impl AuditLog {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            seq: 0,
        }
    }

    fn record(
        &mut self,
        operation: &'static str,
        actor: &str,
        resource: &str,
        allowed: bool,
    ) -> u64 {
        let s = self.seq;
        self.entries.push(AuditEntry {
            operation,
            actor: actor.to_string(),
            resource: resource.to_string(),
            allowed,
            seq: s,
        });
        self.seq += 1;
        s
    }

    fn denied(&self) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| !e.allowed).collect()
    }
    fn count(&self) -> usize {
        self.entries.len()
    }
    fn get(&self, seq: u64) -> Option<&AuditEntry> {
        self.entries.iter().find(|e| e.seq == seq)
    }
}

#[test]
fn test_audit_entry_recorded_on_allow() {
    let mut log = AuditLog::new();
    log.record("tool_call", "user-1", "web_search", true);
    assert_eq!(log.count(), 1);
}

#[test]
fn test_denied_operations_captured() {
    let mut log = AuditLog::new();
    log.record("tool_call", "user-1", "shell_exec", false);
    assert_eq!(log.denied().len(), 1);
}

#[test]
fn test_seq_increments_per_entry() {
    let mut log = AuditLog::new();
    let s0 = log.record("a", "u", "r", true);
    let s1 = log.record("b", "u", "r", true);
    assert_eq!(s1, s0 + 1);
}

#[test]
fn test_retrieve_entry_by_seq() {
    let mut log = AuditLog::new();
    log.record("op", "alice", "file.txt", true);
    let e = log.get(0).unwrap();
    assert_eq!(e.actor, "alice");
    assert_eq!(e.resource, "file.txt");
}

#[test]
fn test_allowed_entries_not_in_denied() {
    let mut log = AuditLog::new();
    log.record("read", "u", "doc", true);
    assert!(log.denied().is_empty());
}

#[test]
fn test_multiple_denied_entries() {
    let mut log = AuditLog::new();
    log.record("read", "u", "r", true);
    log.record("exec", "u", "shell", false);
    log.record("write", "u", "secret", false);
    assert_eq!(log.denied().len(), 2);
}

#[test]
fn test_audit_log_preserves_insertion_order() {
    let mut log = AuditLog::new();
    log.record("first", "u", "r", true);
    log.record("second", "u", "r", true);
    assert_eq!(log.entries[0].operation, "first");
    assert_eq!(log.entries[1].operation, "second");
}
