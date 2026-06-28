use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Expired,
    Revoked,
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SessionState::Active => "ACTIVE",
            SessionState::Expired => "EXPIRED",
            SessionState::Revoked => "REVOKED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct ZeroTrustSession {
    pub id: String,
    pub tenant_id: String,
    pub identity_id: String,
    pub device_id: Option<String>,
    pub state: SessionState,
    pub created_tick: u64,
    pub expires_tick: u64,
    pub last_verified_tick: u64,
    pub metadata: HashMap<String, String>,
}

impl ZeroTrustSession {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        identity_id: impl Into<String>,
        created_tick: u64,
        expires_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            identity_id: identity_id.into(),
            device_id: None,
            state: SessionState::Active,
            created_tick,
            expires_tick,
            last_verified_tick: created_tick,
            metadata: HashMap::new(),
        }
    }

    pub fn with_device(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into()); self
    }

    pub fn is_valid(&self, current_tick: u64) -> bool {
        self.state == SessionState::Active && current_tick < self.expires_tick
    }

    pub fn refresh_verification(&mut self, tick: u64) { self.last_verified_tick = tick; }
    pub fn expire(&mut self) { self.state = SessionState::Expired; }
    pub fn revoke(&mut self) { self.state = SessionState::Revoked; }
}

pub struct SessionStore {
    sessions: HashMap<String, ZeroTrustSession>,
}

impl SessionStore {
    pub fn new() -> Self { Self { sessions: HashMap::new() } }
    pub fn insert(&mut self, s: ZeroTrustSession) { self.sessions.insert(s.id.clone(), s); }
    pub fn get(&self, id: &str) -> Option<&ZeroTrustSession> { self.sessions.get(id) }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ZeroTrustSession> { self.sessions.get_mut(id) }
    pub fn active(&self, current_tick: u64) -> Vec<&ZeroTrustSession> {
        self.sessions.values().filter(|s| s.is_valid(current_tick)).collect()
    }
    pub fn for_identity<'a>(&'a self, identity_id: &str) -> Vec<&'a ZeroTrustSession> {
        self.sessions.values().filter(|s| s.identity_id == identity_id).collect()
    }
    pub fn count(&self) -> usize { self.sessions.len() }
}
