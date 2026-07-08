//! Redaction audit log.
//!
//! Every redaction decision is recorded in an in-memory audit log so that
//! operators can replay why a particular value was scrubbed, which policy
//! fired, and when the decision was made.

/// The reason a field was redacted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedactionReason {
    /// The attribute name matched a drop prefix.
    DropPrefix,
    /// The data class was at or above the redaction threshold.
    ClassificationThreshold,
    /// The attribute name was not on the allowlist.
    NotAllowlisted,
    /// PII was detected in the value.
    PiiDetected,
    /// The opt-in flag for this data type was not set.
    OptInRequired,
    /// The field is in the list of always-sensitive fields.
    SensitiveField,
}

impl RedactionReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            RedactionReason::DropPrefix => "drop_prefix",
            RedactionReason::ClassificationThreshold => "classification_threshold",
            RedactionReason::NotAllowlisted => "not_allowlisted",
            RedactionReason::PiiDetected => "pii_detected",
            RedactionReason::OptInRequired => "opt_in_required",
            RedactionReason::SensitiveField => "sensitive_field",
        }
    }
}

/// A single redaction decision recorded in the audit log.
#[derive(Debug, Clone)]
pub struct RedactionEntry {
    /// Monotonic sequence number.
    pub seq: u64,
    /// Name of the attribute that was redacted.
    pub attr_name: String,
    /// The reason for redaction.
    pub reason: RedactionReason,
    /// The policy module that made the decision.
    pub policy: String,
    /// Whether the value was replaced (true) or dropped (false).
    pub replaced: bool,
}

/// In-memory audit log for redaction decisions.
#[derive(Debug, Default)]
pub struct RedactionAuditLog {
    entries: Vec<RedactionEntry>,
    next_seq: u64,
}

impl RedactionAuditLog {
    /// Create a new empty audit log.
    pub fn new() -> Self {
        RedactionAuditLog {
            entries: Vec::new(),
            next_seq: 0,
        }
    }

    /// Record a redaction decision.
    pub fn record(
        &mut self,
        attr_name: impl Into<String>,
        reason: RedactionReason,
        policy: impl Into<String>,
        replaced: bool,
    ) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.push(RedactionEntry {
            seq,
            attr_name: attr_name.into(),
            reason,
            policy: policy.into(),
            replaced,
        });
        seq
    }

    /// Iterate over all audit entries.
    pub fn entries(&self) -> &[RedactionEntry] {
        &self.entries
    }

    /// Return how many redactions have been recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no entries have been recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Look up an entry by sequence number.
    pub fn get(&self, seq: u64) -> Option<&RedactionEntry> {
        self.entries.iter().find(|e| e.seq == seq)
    }

    /// Filter entries by reason.
    pub fn by_reason(&self, reason: &RedactionReason) -> Vec<&RedactionEntry> {
        self.entries
            .iter()
            .filter(|e| &e.reason == reason)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_retrieve() {
        let mut log = RedactionAuditLog::new();
        let seq = log.record(
            "user.email",
            RedactionReason::SensitiveField,
            "log_policy",
            true,
        );
        assert_eq!(seq, 0);
        let entry = log.get(seq).unwrap();
        assert_eq!(entry.attr_name, "user.email");
        assert_eq!(entry.reason, RedactionReason::SensitiveField);
    }

    #[test]
    fn seq_increments() {
        let mut log = RedactionAuditLog::new();
        let s1 = log.record("a", RedactionReason::PiiDetected, "pii", true);
        let s2 = log.record("b", RedactionReason::OptInRequired, "opt_in", false);
        assert_eq!(s1, 0);
        assert_eq!(s2, 1);
    }

    #[test]
    fn filter_by_reason() {
        let mut log = RedactionAuditLog::new();
        log.record("a", RedactionReason::PiiDetected, "p", true);
        log.record("b", RedactionReason::DropPrefix, "p", false);
        log.record("c", RedactionReason::PiiDetected, "p", true);
        let pii = log.by_reason(&RedactionReason::PiiDetected);
        assert_eq!(pii.len(), 2);
    }
}
