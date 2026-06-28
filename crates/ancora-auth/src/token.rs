use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenKind {
    Bearer,
    ServiceAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub raw: String,
    pub kind: TokenKind,
    pub subject: String,
    pub tenant_id: String,
    pub expires_at_tick: u64,
    pub scopes: Vec<String>,
    pub revoked: bool,
}

impl Token {
    pub fn new(
        raw: impl Into<String>,
        kind: TokenKind,
        subject: impl Into<String>,
        tenant_id: impl Into<String>,
        expires_at_tick: u64,
        scopes: Vec<String>,
    ) -> Self {
        Self {
            raw: raw.into(),
            kind,
            subject: subject.into(),
            tenant_id: tenant_id.into(),
            expires_at_tick,
            scopes,
            revoked: false,
        }
    }

    pub fn is_expired(&self, current_tick: u64) -> bool {
        current_tick >= self.expires_at_tick
    }

    pub fn is_valid(&self, current_tick: u64) -> bool {
        !self.revoked && !self.is_expired(current_tick)
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
}
