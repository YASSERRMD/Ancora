use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreatIntelAction {
    IndicatorAdded,
    IndicatorExpired,
    IndicatorDeactivated,
    FeedIngested,
    FeedEnabled,
    FeedDisabled,
    ScoreComputed,
    AlertTriggered,
}

impl fmt::Display for ThreatIntelAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ThreatIntelAction::IndicatorAdded => "INDICATOR_ADDED",
            ThreatIntelAction::IndicatorExpired => "INDICATOR_EXPIRED",
            ThreatIntelAction::IndicatorDeactivated => "INDICATOR_DEACTIVATED",
            ThreatIntelAction::FeedIngested => "FEED_INGESTED",
            ThreatIntelAction::FeedEnabled => "FEED_ENABLED",
            ThreatIntelAction::FeedDisabled => "FEED_DISABLED",
            ThreatIntelAction::ScoreComputed => "SCORE_COMPUTED",
            ThreatIntelAction::AlertTriggered => "ALERT_TRIGGERED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ThreatIntelAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub action: ThreatIntelAction,
    pub subject: String,
    pub detail: String,
}

impl ThreatIntelAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        action: ThreatIntelAction,
        subject: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            action,
            subject: subject.into(),
            detail: detail.into(),
        }
    }
}

pub struct ThreatIntelAuditLog {
    entries: VecDeque<ThreatIntelAuditEntry>,
}

impl Default for ThreatIntelAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatIntelAuditLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::new(),
        }
    }
    pub fn record(&mut self, entry: ThreatIntelAuditEntry) {
        self.entries.push_back(entry);
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a ThreatIntelAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }
    pub fn by_action<'a>(&'a self, action: &ThreatIntelAction) -> Vec<&'a ThreatIntelAuditEntry> {
        self.entries
            .iter()
            .filter(|e| &e.action == action)
            .collect()
    }
    pub fn all(&self) -> impl Iterator<Item = &ThreatIntelAuditEntry> {
        self.entries.iter()
    }
}
