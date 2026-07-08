/// Plugin audit log.
///
/// Every significant plugin lifecycle event is recorded in the audit log,
/// providing an immutable trail for compliance, debugging, and security review.

/// The kind of event being recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// The plugin was successfully loaded.
    Loaded,
    /// The plugin was unloaded cleanly.
    Unloaded,
    /// The plugin crashed or panicked.
    Crashed,
    /// A policy violation was detected (network, filesystem, resource, etc.).
    PolicyViolation,
    /// The plugin's signature was verified.
    SignatureVerified,
    /// The plugin's signature verification failed.
    SignatureFailed,
    /// A custom event for extensibility.
    Custom(String),
}

/// A single audit log entry.
#[derive(Debug, Clone)]
pub struct AuditEvent {
    /// The identifier of the plugin that generated this event.
    pub plugin_id: String,
    /// The kind of event.
    pub kind: EventKind,
    /// A human-readable description of the event.
    pub detail: String,
}

/// An append-only audit log for plugin events.
#[derive(Debug, Default)]
pub struct AuditLog {
    entries: Vec<AuditEvent>,
}

impl AuditLog {
    /// Create a new, empty audit log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append an event to the log.
    pub fn record(&mut self, event: AuditEvent) {
        self.entries.push(event);
    }

    /// Returns all entries in insertion order.
    pub fn entries(&self) -> &[AuditEvent] {
        &self.entries
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no events have been recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns all events for a specific plugin.
    pub fn events_for_plugin<'a>(&'a self, plugin_id: &str) -> Vec<&'a AuditEvent> {
        self.entries
            .iter()
            .filter(|e| e.plugin_id == plugin_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_log_has_zero_entries() {
        let log = AuditLog::new();
        assert_eq!(log.len(), 0);
        assert!(log.is_empty());
    }

    #[test]
    fn record_and_retrieve_event() {
        let mut log = AuditLog::new();
        log.record(AuditEvent {
            plugin_id: "test-plugin".into(),
            kind: EventKind::Loaded,
            detail: "wasm".into(),
        });
        assert_eq!(log.len(), 1);
        assert_eq!(log.entries()[0].plugin_id, "test-plugin");
    }

    #[test]
    fn filter_by_plugin_returns_correct_events() {
        let mut log = AuditLog::new();
        log.record(AuditEvent {
            plugin_id: "a".into(),
            kind: EventKind::Loaded,
            detail: "".into(),
        });
        log.record(AuditEvent {
            plugin_id: "b".into(),
            kind: EventKind::Loaded,
            detail: "".into(),
        });
        let a_events = log.events_for_plugin("a");
        assert_eq!(a_events.len(), 1);
    }
}
