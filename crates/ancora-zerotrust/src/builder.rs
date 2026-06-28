use crate::identity::{Identity, IdentityKind};
use crate::request::AccessRequest;
use crate::session::ZeroTrustSession;

pub struct IdentityBuilder {
    id: String,
    tenant_id: String,
    kind: IdentityKind,
    tick: u64,
    groups: Vec<String>,
}

impl IdentityBuilder {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, kind: IdentityKind) -> Self {
        Self { id: id.into(), tenant_id: tenant_id.into(), kind, tick: 0, groups: Vec::new() }
    }
    pub fn tick(mut self, t: u64) -> Self { self.tick = t; self }
    pub fn group(mut self, g: impl Into<String>) -> Self { self.groups.push(g.into()); self }
    pub fn build(self) -> Identity {
        let mut ident = Identity::new(self.id, self.tenant_id, self.kind, self.tick);
        for g in self.groups { ident.add_group(g); }
        ident
    }
}

pub struct SessionBuilder {
    id: String,
    tenant_id: String,
    identity_id: String,
    created_tick: u64,
    expires_tick: u64,
    device_id: Option<String>,
}

impl SessionBuilder {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, identity_id: impl Into<String>) -> Self {
        Self { id: id.into(), tenant_id: tenant_id.into(), identity_id: identity_id.into(), created_tick: 0, expires_tick: 3600, device_id: None }
    }
    pub fn created_at(mut self, t: u64) -> Self { self.created_tick = t; self }
    pub fn expires_at(mut self, t: u64) -> Self { self.expires_tick = t; self }
    pub fn device(mut self, d: impl Into<String>) -> Self { self.device_id = Some(d.into()); self }
    pub fn build(self) -> ZeroTrustSession {
        let mut s = ZeroTrustSession::new(self.id, self.tenant_id, self.identity_id, self.created_tick, self.expires_tick);
        if let Some(d) = self.device_id { s = s.with_device(d); }
        s
    }
}

pub fn make_request(id: impl Into<String>, tenant_id: impl Into<String>, identity_id: impl Into<String>, resource: impl Into<String>, action: impl Into<String>, tick: u64) -> AccessRequest {
    AccessRequest::new(id, tenant_id, identity_id, resource, action, tick)
}
