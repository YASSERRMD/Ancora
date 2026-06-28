use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthEvent {
    OidcLoginSuccess { subject: String, tenant_id: String, tick: u64 },
    OidcLoginFailure { tenant_id: String, reason: String, tick: u64 },
    SamlLoginSuccess { subject: String, tenant_id: String, tick: u64 },
    SamlLoginFailure { tenant_id: String, reason: String, tick: u64 },
    ServiceAccountAuth { account_id: String, tenant_id: String, tick: u64 },
    TokenRevoked { token_prefix: String, tick: u64 },
    SessionLogout { session_id: String, tick: u64 },
    MfaVerified { subject: String, method: String, tick: u64 },
    MfaFailed { subject: String, tick: u64 },
}

#[derive(Debug, Default)]
pub struct AuthAuditLog {
    events: VecDeque<AuthEvent>,
    max_size: usize,
}

impl AuthAuditLog {
    pub fn new(max_size: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_size,
        }
    }

    pub fn record(&mut self, event: AuthEvent) {
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn events(&self) -> impl Iterator<Item = &AuthEvent> {
        self.events.iter()
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }

    pub fn failures_for_tenant(&self, tenant_id: &str) -> Vec<&AuthEvent> {
        self.events
            .iter()
            .filter(|e| match e {
                AuthEvent::OidcLoginFailure { tenant_id: t, .. } => t == tenant_id,
                AuthEvent::SamlLoginFailure { tenant_id: t, .. } => t == tenant_id,
                _ => false,
            })
            .collect()
    }
}
