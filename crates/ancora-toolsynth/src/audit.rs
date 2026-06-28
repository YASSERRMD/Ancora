/// A single audit event for tool synthesis.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub tick: u64,
    pub event: AuditEvent,
}

#[derive(Debug, Clone)]
pub enum AuditEvent {
    Synthesized { tool_name: String, goal: String },
    Approved { tool_name: String, approver: String },
    Revoked { tool_name: String },
    Executed { tool_name: String },
    Cached { tool_name: String },
}

/// Append-only audit trail for synthesized tools.
#[derive(Debug, Default)]
pub struct SynthAudit {
    entries: Vec<AuditEntry>,
}

impl SynthAudit {
    pub fn record(&mut self, tick: u64, event: AuditEvent) {
        self.entries.push(AuditEntry { tick, event });
    }

    pub fn entries(&self) -> &[AuditEntry] {
        &self.entries
    }

    pub fn events_for_tool(&self, name: &str) -> Vec<&AuditEvent> {
        self.entries
            .iter()
            .filter(|e| match &e.event {
                AuditEvent::Synthesized { tool_name, .. } => tool_name == name,
                AuditEvent::Approved { tool_name, .. } => tool_name == name,
                AuditEvent::Revoked { tool_name } => tool_name == name,
                AuditEvent::Executed { tool_name } => tool_name == name,
                AuditEvent::Cached { tool_name } => tool_name == name,
            })
            .map(|e| &e.event)
            .collect()
    }
}
