use std::fmt;
use crate::incident::Severity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscalationChannel {
    Pager,
    Email,
    Chat,
    Phone,
}

impl fmt::Display for EscalationChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EscalationChannel::Pager => "PAGER",
            EscalationChannel::Email => "EMAIL",
            EscalationChannel::Chat => "CHAT",
            EscalationChannel::Phone => "PHONE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct EscalationLevel {
    pub level: u8,
    pub on_call: String,
    pub channel: EscalationChannel,
    pub delay_ticks: u64,
}

impl EscalationLevel {
    pub fn new(level: u8, on_call: impl Into<String>, channel: EscalationChannel, delay_ticks: u64) -> Self {
        Self { level, on_call: on_call.into(), channel, delay_ticks }
    }
}

#[derive(Debug, Clone)]
pub struct EscalationPolicy {
    pub tenant_id: String,
    pub min_severity: Severity,
    pub levels: Vec<EscalationLevel>,
}

impl EscalationPolicy {
    pub fn new(tenant_id: impl Into<String>, min_severity: Severity) -> Self {
        Self { tenant_id: tenant_id.into(), min_severity, levels: Vec::new() }
    }

    pub fn add_level(&mut self, level: EscalationLevel) { self.levels.push(level); }

    pub fn should_escalate(&self, severity: &Severity) -> bool {
        severity >= &self.min_severity
    }

    pub fn level_count(&self) -> usize { self.levels.len() }

    pub fn level_at(&self, index: usize) -> Option<&EscalationLevel> {
        self.levels.get(index)
    }
}

#[derive(Debug, Clone)]
pub struct EscalationRecord {
    pub incident_id: String,
    pub level: u8,
    pub on_call: String,
    pub channel: EscalationChannel,
    pub tick: u64,
}

impl EscalationRecord {
    pub fn new(incident_id: impl Into<String>, level: u8, on_call: impl Into<String>, channel: EscalationChannel, tick: u64) -> Self {
        Self { incident_id: incident_id.into(), level, on_call: on_call.into(), channel, tick }
    }
}
