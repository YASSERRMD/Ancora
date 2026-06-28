use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirGapAction {
    TransferRequested,
    TransferApproved,
    TransferRejected,
    TransferCompleted,
    TransferCancelled,
    PolicyEvaluated,
    ZoneCreated,
    ProcedureStarted,
    ProcedureStepCompleted,
    ProcedureCompleted,
    MediaBlocked,
}

impl fmt::Display for AirGapAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AirGapAction::TransferRequested => "TRANSFER_REQUESTED",
            AirGapAction::TransferApproved => "TRANSFER_APPROVED",
            AirGapAction::TransferRejected => "TRANSFER_REJECTED",
            AirGapAction::TransferCompleted => "TRANSFER_COMPLETED",
            AirGapAction::TransferCancelled => "TRANSFER_CANCELLED",
            AirGapAction::PolicyEvaluated => "POLICY_EVALUATED",
            AirGapAction::ZoneCreated => "ZONE_CREATED",
            AirGapAction::ProcedureStarted => "PROCEDURE_STARTED",
            AirGapAction::ProcedureStepCompleted => "PROCEDURE_STEP_COMPLETED",
            AirGapAction::ProcedureCompleted => "PROCEDURE_COMPLETED",
            AirGapAction::MediaBlocked => "MEDIA_BLOCKED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct AirGapAuditEntry {
    pub tick: u64,
    pub tenant_id: String,
    pub action: AirGapAction,
    pub actor: String,
    pub detail: String,
}

impl AirGapAuditEntry {
    pub fn new(
        tick: u64,
        tenant_id: impl Into<String>,
        action: AirGapAction,
        actor: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            tick,
            tenant_id: tenant_id.into(),
            action,
            actor: actor.into(),
            detail: detail.into(),
        }
    }
}

pub struct AirGapAuditLog {
    entries: VecDeque<AirGapAuditEntry>,
}

impl AirGapAuditLog {
    pub fn new() -> Self { Self { entries: VecDeque::new() } }

    pub fn record(&mut self, entry: AirGapAuditEntry) { self.entries.push_back(entry); }

    pub fn count(&self) -> usize { self.entries.len() }

    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a AirGapAuditEntry> {
        self.entries.iter().filter(|e| e.tenant_id == tenant_id).collect()
    }

    pub fn by_action<'a>(&'a self, action: &AirGapAction) -> Vec<&'a AirGapAuditEntry> {
        self.entries.iter().filter(|e| &e.action == action).collect()
    }

    pub fn all(&self) -> impl Iterator<Item = &AirGapAuditEntry> { self.entries.iter() }
}
