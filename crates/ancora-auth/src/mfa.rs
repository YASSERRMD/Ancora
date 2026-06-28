use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MfaMethod {
    Totp,
    HardwareKey,
    PushNotification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MfaStatus {
    Pending,
    Verified,
    Failed,
    Bypassed,
}

#[derive(Debug, Clone)]
pub struct MfaChallenge {
    pub challenge_id: String,
    pub subject: String,
    pub method: MfaMethod,
    pub status: MfaStatus,
    pub issued_at_tick: u64,
    pub expires_at_tick: u64,
    pub expected_code: String,
}

impl MfaChallenge {
    pub fn new(
        challenge_id: impl Into<String>,
        subject: impl Into<String>,
        method: MfaMethod,
        expected_code: impl Into<String>,
        issued_at_tick: u64,
        ttl_ticks: u64,
    ) -> Self {
        Self {
            challenge_id: challenge_id.into(),
            subject: subject.into(),
            method,
            status: MfaStatus::Pending,
            issued_at_tick,
            expires_at_tick: issued_at_tick + ttl_ticks,
            expected_code: expected_code.into(),
        }
    }

    pub fn verify(&mut self, code: &str, current_tick: u64) -> bool {
        if current_tick >= self.expires_at_tick {
            self.status = MfaStatus::Failed;
            return false;
        }
        if code == self.expected_code {
            self.status = MfaStatus::Verified;
            true
        } else {
            self.status = MfaStatus::Failed;
            false
        }
    }

    pub fn is_verified(&self) -> bool {
        self.status == MfaStatus::Verified
    }
}

#[derive(Debug, Default)]
pub struct MfaEnforcer {
    challenges: HashMap<String, MfaChallenge>,
    required_for_tenants: std::collections::HashSet<String>,
}

impl MfaEnforcer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn require_for_tenant(&mut self, tenant_id: impl Into<String>) {
        self.required_for_tenants.insert(tenant_id.into());
    }

    pub fn is_required(&self, tenant_id: &str) -> bool {
        self.required_for_tenants.contains(tenant_id)
    }

    pub fn issue_challenge(&mut self, challenge: MfaChallenge) {
        self.challenges.insert(challenge.challenge_id.clone(), challenge);
    }

    pub fn verify_challenge(&mut self, challenge_id: &str, code: &str, current_tick: u64) -> bool {
        match self.challenges.get_mut(challenge_id) {
            Some(c) => c.verify(code, current_tick),
            None => false,
        }
    }

    pub fn get_challenge(&self, challenge_id: &str) -> Option<&MfaChallenge> {
        self.challenges.get(challenge_id)
    }
}
