use crate::attach::FeedbackStore;
use crate::schema::{Feedback, ThumbsRating};

/// Simulate an audit log by verifying all feedback records are traceable.
#[derive(Debug)]
struct AuditLog {
    entries: Vec<String>,
}

impl AuditLog {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn record(&mut self, feedback: &Feedback) {
        self.entries.push(format!(
            "AUDIT: feedback={} run={} author={} rating={:?}",
            feedback.id, feedback.run_id, feedback.author, feedback.rating
        ));
    }

    fn contains(&self, feedback_id: &str) -> bool {
        self.entries.iter().any(|e| e.contains(&format!("feedback={}", feedback_id)))
    }
}

#[test]
fn feedback_audited() {
    let mut store = FeedbackStore::new();
    let mut audit = AuditLog::new();

    let records = vec![
        Feedback::new("f1", "run-1", None, ThumbsRating::Up, None, "alice", 0),
        Feedback::new("f2", "run-1", None, ThumbsRating::Down, Some("incorrect".into()), "bob", 1),
        Feedback::new("f3", "run-2", None, ThumbsRating::Up, None, "carol", 2),
    ];

    for fb in &records {
        audit.record(fb);
        store.attach(fb.clone());
    }

    // All feedback is audited
    assert!(audit.contains("f1"));
    assert!(audit.contains("f2"));
    assert!(audit.contains("f3"));
    assert_eq!(audit.entries.len(), 3);

    // All feedback is stored
    assert_eq!(store.for_run("run-1").len(), 2);
    assert_eq!(store.for_run("run-2").len(), 1);
}

#[test]
fn audit_log_entries_include_author() {
    let mut audit = AuditLog::new();
    let fb = Feedback::new("f99", "run-99", None, ThumbsRating::Down, None, "mallory", 0);
    audit.record(&fb);
    assert!(audit.entries[0].contains("author=mallory"));
}
