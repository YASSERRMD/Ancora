use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Active,
    LoggedOut,
    Expired,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: String,
    pub tenant_id: String,
    pub subject: String,
    pub created_at_tick: u64,
    pub expires_at_tick: u64,
    pub token_raw: String,
    pub state: SessionState,
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(
        session_id: impl Into<String>,
        tenant_id: impl Into<String>,
        subject: impl Into<String>,
        token_raw: impl Into<String>,
        created_at_tick: u64,
        expires_at_tick: u64,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            tenant_id: tenant_id.into(),
            subject: subject.into(),
            created_at_tick,
            expires_at_tick,
            token_raw: token_raw.into(),
            state: SessionState::Active,
            metadata: HashMap::new(),
        }
    }

    pub fn is_valid(&self, current_tick: u64) -> bool {
        self.state == SessionState::Active && current_tick < self.expires_at_tick
    }

    pub fn logout(&mut self) {
        self.state = SessionState::LoggedOut;
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Default)]
pub struct SessionStore {
    sessions: HashMap<String, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self, session: Session) {
        self.sessions.insert(session.session_id.clone(), session);
    }

    pub fn get(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    pub fn logout(&mut self, session_id: &str) -> bool {
        match self.sessions.get_mut(session_id) {
            Some(s) => {
                s.logout();
                true
            }
            None => false,
        }
    }

    pub fn active_count(&self, current_tick: u64) -> usize {
        self.sessions
            .values()
            .filter(|s| s.is_valid(current_tick))
            .count()
    }

    pub fn sessions_for_subject(&self, subject: &str) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| s.subject == subject)
            .collect()
    }
}
