use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntrospectStatus {
    Active,
    Expired,
    Revoked,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct IntrospectResult {
    pub status: IntrospectStatus,
    pub subject: Option<String>,
    pub tenant_id: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at_tick: Option<u64>,
}

impl IntrospectResult {
    pub fn inactive() -> Self {
        Self {
            status: IntrospectStatus::Unknown,
            subject: None,
            tenant_id: None,
            scopes: Vec::new(),
            expires_at_tick: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == IntrospectStatus::Active
    }
}

pub struct TokenIntrospector {
    tokens: std::collections::HashMap<String, Token>,
}

impl TokenIntrospector {
    pub fn new() -> Self {
        Self {
            tokens: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, token: Token) {
        self.tokens.insert(token.raw.clone(), token);
    }

    pub fn introspect(&self, raw: &str, current_tick: u64) -> IntrospectResult {
        match self.tokens.get(raw) {
            None => IntrospectResult::inactive(),
            Some(t) => {
                let status = if t.revoked {
                    IntrospectStatus::Revoked
                } else if t.is_expired(current_tick) {
                    IntrospectStatus::Expired
                } else {
                    IntrospectStatus::Active
                };
                IntrospectResult {
                    status,
                    subject: Some(t.subject.clone()),
                    tenant_id: Some(t.tenant_id.clone()),
                    scopes: t.scopes.clone(),
                    expires_at_tick: Some(t.expires_at_tick),
                }
            }
        }
    }
}

impl Default for TokenIntrospector {
    fn default() -> Self {
        Self::new()
    }
}
