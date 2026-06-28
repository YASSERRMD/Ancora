use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditEventKind {
    PolicyDecision,
    AccessGranted,
    AccessDenied,
    AdminAction,
    TenantCreated,
    TenantSuspended,
    SecretRotated,
    RunStarted,
    RunCompleted,
}

/// A signed, immutable audit event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp_secs: u64,
    pub kind: AuditEventKind,
    pub tenant_id: String,
    pub actor: String,
    pub resource: String,
    pub decision: String,
    pub signature: String,
}

impl AuditEvent {
    pub fn new(
        timestamp_secs: u64,
        kind: AuditEventKind,
        tenant_id: impl Into<String>,
        actor: impl Into<String>,
        resource: impl Into<String>,
        decision: impl Into<String>,
        signing_key: &[u8],
    ) -> Self {
        let tenant_id = tenant_id.into();
        let actor = actor.into();
        let resource = resource.into();
        let decision = decision.into();
        let signature = sign(&tenant_id, &actor, &resource, &decision, timestamp_secs, signing_key);
        Self { timestamp_secs, kind, tenant_id, actor, resource, decision, signature }
    }

    pub fn verify(&self, signing_key: &[u8]) -> bool {
        let expected = sign(
            &self.tenant_id,
            &self.actor,
            &self.resource,
            &self.decision,
            self.timestamp_secs,
            signing_key,
        );
        self.signature == expected
    }
}

fn sign(tenant: &str, actor: &str, resource: &str, decision: &str, ts: u64, key: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(key);
    h.update(tenant.as_bytes());
    h.update(actor.as_bytes());
    h.update(resource.as_bytes());
    h.update(decision.as_bytes());
    h.update(ts.to_le_bytes());
    hex::encode(h.finalize())
}

/// Append-only audit channel.
pub struct AuditChannel {
    events: Vec<AuditEvent>,
}

impl AuditChannel {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }

    /// Query events by tenant and kind.
    pub fn query(&self, tenant_id: &str, kind: &AuditEventKind) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.tenant_id == tenant_id && &e.kind == kind)
            .collect()
    }
}

impl Default for AuditChannel {
    fn default() -> Self {
        Self::new()
    }
}
