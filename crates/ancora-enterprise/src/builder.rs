use crate::checkpoint::{CheckStatus, HealthCheck};
use crate::incident::{EnterpriseIncident, IncidentSeverity};
use crate::license::{EnterpriseCap, EnterpriseLicense, LicenseTier};
use crate::posture::DomainScore;

pub struct LicenseBuilder {
    id: String,
    tenant_id: String,
    tier: LicenseTier,
    max_users: u32,
    max_tenants: u32,
    tick: u64,
    caps: Vec<EnterpriseCap>,
    expires: Option<u64>,
}

impl LicenseBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        tier: LicenseTier,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            tier,
            max_users: 100,
            max_tenants: 1,
            tick,
            caps: Vec::new(),
            expires: None,
        }
    }

    pub fn max_users(mut self, n: u32) -> Self { self.max_users = n; self }
    pub fn max_tenants(mut self, n: u32) -> Self { self.max_tenants = n; self }
    pub fn cap(mut self, c: EnterpriseCap) -> Self { self.caps.push(c); self }
    pub fn expires(mut self, tick: u64) -> Self { self.expires = Some(tick); self }

    pub fn build(self) -> EnterpriseLicense {
        let mut lic = EnterpriseLicense::new(self.id, self.tenant_id, self.tier, self.max_users, self.max_tenants, self.tick);
        for c in self.caps { lic = lic.with_cap(c); }
        if let Some(e) = self.expires { lic = lic.with_expiry(e); }
        lic
    }
}

pub struct IncidentBuilder {
    id: String,
    tenant_id: String,
    title: String,
    severity: IncidentSeverity,
    domain: String,
    tick: u64,
    assignee: Option<String>,
}

impl IncidentBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        title: impl Into<String>,
        severity: IncidentSeverity,
        domain: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self { id: id.into(), tenant_id: tenant_id.into(), title: title.into(), severity, domain: domain.into(), tick, assignee: None }
    }

    pub fn assignee(mut self, a: impl Into<String>) -> Self { self.assignee = Some(a.into()); self }

    pub fn build(self) -> EnterpriseIncident {
        let mut inc = EnterpriseIncident::new(self.id, self.tenant_id, self.title, self.severity, self.domain, self.tick);
        if let Some(a) = self.assignee { inc = inc.with_assignee(a); }
        inc
    }
}

pub struct HealthCheckBuilder {
    id: String,
    name: String,
    domain: String,
    status: CheckStatus,
    message: String,
    tick: u64,
}

impl HealthCheckBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>, domain: impl Into<String>, status: CheckStatus, tick: u64) -> Self {
        Self { id: id.into(), name: name.into(), domain: domain.into(), status, message: String::new(), tick }
    }

    pub fn message(mut self, m: impl Into<String>) -> Self { self.message = m.into(); self }

    pub fn build(self) -> HealthCheck {
        HealthCheck::new(self.id, self.name, self.domain, self.status, self.message, self.tick)
    }
}

pub struct DomainScoreBuilder {
    domain: String,
    score: u8,
    findings: u32,
    critical: u32,
}

impl DomainScoreBuilder {
    pub fn new(domain: impl Into<String>, score: u8) -> Self {
        Self { domain: domain.into(), score, findings: 0, critical: 0 }
    }

    pub fn findings(mut self, n: u32) -> Self { self.findings = n; self }
    pub fn critical(mut self, n: u32) -> Self { self.critical = n; self }
    pub fn build(self) -> DomainScore { DomainScore::new(self.domain, self.score, self.findings, self.critical) }
}
