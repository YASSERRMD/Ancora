use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantEventKind {
    Registered,
    Activated,
    Suspended,
    Deleted,
    QuotaUpdated,
    NamespaceKeySet,
    NamespaceKeyRemoved,
}

#[derive(Debug, Clone)]
pub struct TenantEvent {
    pub tick: u64,
    pub tenant_id: String,
    pub kind: TenantEventKind,
    pub detail: Option<String>,
}

impl TenantEvent {
    pub fn new(tick: u64, tenant_id: impl Into<String>, kind: TenantEventKind) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            kind,
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

#[derive(Debug, Default)]
pub struct TenantEventLog {
    events: VecDeque<TenantEvent>,
}

impl TenantEventLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, event: TenantEvent) {
        self.events.push_back(event);
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }

    pub fn events_for(&self, tenant_id: &str) -> Vec<&TenantEvent> {
        self.events
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }

    pub fn events_of_kind(&self, kind: &TenantEventKind) -> Vec<&TenantEvent> {
        self.events.iter().filter(|e| &e.kind == kind).collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &TenantEvent> {
        self.events.iter()
    }
}
