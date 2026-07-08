use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Open,
    LoggedIn,
    LoggedOut,
    Closed,
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SessionState::Open => "OPEN",
            SessionState::LoggedIn => "LOGGED_IN",
            SessionState::LoggedOut => "LOGGED_OUT",
            SessionState::Closed => "CLOSED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct HsmSession {
    pub id: u64,
    pub slot_id: u32,
    pub state: SessionState,
    pub read_write: bool,
    pub created_tick: u64,
    pub metadata: HashMap<String, String>,
}

impl HsmSession {
    pub fn new(id: u64, slot_id: u32, read_write: bool, tick: u64) -> Self {
        Self {
            id,
            slot_id,
            state: SessionState::Open,
            read_write,
            created_tick: tick,
            metadata: HashMap::new(),
        }
    }
    pub fn login(&mut self) {
        self.state = SessionState::LoggedIn;
    }
    pub fn logout(&mut self) {
        self.state = SessionState::LoggedOut;
    }
    pub fn close(&mut self) {
        self.state = SessionState::Closed;
    }
    pub fn is_active(&self) -> bool {
        !matches!(self.state, SessionState::Closed)
    }
    pub fn is_logged_in(&self) -> bool {
        self.state == SessionState::LoggedIn
    }
}

pub struct SessionManager {
    sessions: HashMap<u64, HsmSession>,
    next_id: u64,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn open_session(&mut self, slot_id: u32, read_write: bool, tick: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.sessions
            .insert(id, HsmSession::new(id, slot_id, read_write, tick));
        id
    }

    pub fn get(&self, id: u64) -> Option<&HsmSession> {
        self.sessions.get(&id)
    }
    pub fn get_mut(&mut self, id: u64) -> Option<&mut HsmSession> {
        self.sessions.get_mut(&id)
    }

    pub fn close_session(&mut self, id: u64) {
        if let Some(s) = self.sessions.get_mut(&id) {
            s.close();
        }
    }

    pub fn active(&self) -> Vec<&HsmSession> {
        self.sessions.values().filter(|s| s.is_active()).collect()
    }
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}
