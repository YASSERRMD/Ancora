// Policy: GDPR right-to-erasure -- user data purged from journal on request.

struct JournalStore {
    entries: Vec<(String, String)>, // (user_id, payload)
}

impl JournalStore {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    fn add(&mut self, user_id: &str, payload: &str) {
        self.entries
            .push((user_id.to_string(), payload.to_string()));
    }
    fn erase_user(&mut self, user_id: &str) -> usize {
        let before = self.entries.len();
        self.entries.retain(|(uid, _)| uid != user_id);
        before - self.entries.len()
    }
    fn count_for(&self, user_id: &str) -> usize {
        self.entries
            .iter()
            .filter(|(uid, _)| uid == user_id)
            .count()
    }
}

#[test]
fn test_erase_removes_user_entries() {
    let mut s = JournalStore::new();
    s.add("alice", "run-1");
    s.add("alice", "run-2");
    s.add("bob", "run-3");
    s.erase_user("alice");
    assert_eq!(s.count_for("alice"), 0);
    assert_eq!(s.count_for("bob"), 1);
}

#[test]
fn test_erase_returns_deleted_count() {
    let mut s = JournalStore::new();
    s.add("alice", "a");
    s.add("alice", "b");
    s.add("alice", "c");
    let deleted = s.erase_user("alice");
    assert_eq!(deleted, 3);
}

#[test]
fn test_erase_nonexistent_user_returns_zero() {
    let mut s = JournalStore::new();
    s.add("alice", "run");
    let deleted = s.erase_user("charlie");
    assert_eq!(deleted, 0);
}

#[test]
fn test_other_users_not_affected_by_erasure() {
    let mut s = JournalStore::new();
    s.add("alice", "a1");
    s.add("bob", "b1");
    s.add("charlie", "c1");
    s.erase_user("alice");
    assert_eq!(s.count_for("bob"), 1);
    assert_eq!(s.count_for("charlie"), 1);
}

#[test]
fn test_double_erasure_is_safe() {
    let mut s = JournalStore::new();
    s.add("alice", "a");
    s.erase_user("alice");
    let second = s.erase_user("alice");
    assert_eq!(second, 0);
}

#[test]
fn test_total_entries_reduced_after_erasure() {
    let mut s = JournalStore::new();
    for _ in 0..5 {
        s.add("alice", "x");
    }
    for _ in 0..3 {
        s.add("bob", "y");
    }
    s.erase_user("alice");
    assert_eq!(s.entries.len(), 3);
}
