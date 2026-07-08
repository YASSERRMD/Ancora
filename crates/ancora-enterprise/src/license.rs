use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseTier {
    Community,
    Professional,
    Enterprise,
    GovCloud,
}

impl fmt::Display for LicenseTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LicenseTier::Community => "COMMUNITY",
            LicenseTier::Professional => "PROFESSIONAL",
            LicenseTier::Enterprise => "ENTERPRISE",
            LicenseTier::GovCloud => "GOV_CLOUD",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnterpriseCap {
    Hsm,
    AirGap,
    RedTeamSim,
    PentestAutomation,
    AdvancedCompliance,
    SsoIntegration,
    AuditExport,
    MultiRegion,
    CustomRoles,
    ThreatIntelFeed,
}

impl fmt::Display for EnterpriseCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnterpriseCap::Hsm => "HSM",
            EnterpriseCap::AirGap => "AIR_GAP",
            EnterpriseCap::RedTeamSim => "RED_TEAM_SIM",
            EnterpriseCap::PentestAutomation => "PENTEST_AUTOMATION",
            EnterpriseCap::AdvancedCompliance => "ADVANCED_COMPLIANCE",
            EnterpriseCap::SsoIntegration => "SSO_INTEGRATION",
            EnterpriseCap::AuditExport => "AUDIT_EXPORT",
            EnterpriseCap::MultiRegion => "MULTI_REGION",
            EnterpriseCap::CustomRoles => "CUSTOM_ROLES",
            EnterpriseCap::ThreatIntelFeed => "THREAT_INTEL_FEED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct EnterpriseLicense {
    pub id: String,
    pub tenant_id: String,
    pub tier: LicenseTier,
    pub max_users: u32,
    pub max_tenants: u32,
    pub issued_tick: u64,
    pub expires_tick: Option<u64>,
    capabilities: HashSet<EnterpriseCap>,
}

impl EnterpriseLicense {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        tier: LicenseTier,
        max_users: u32,
        max_tenants: u32,
        issued_tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            tier,
            max_users,
            max_tenants,
            issued_tick,
            expires_tick: None,
            capabilities: HashSet::new(),
        }
    }

    pub fn with_cap(mut self, cap: EnterpriseCap) -> Self {
        self.capabilities.insert(cap);
        self
    }

    pub fn with_expiry(mut self, tick: u64) -> Self {
        self.expires_tick = Some(tick);
        self
    }

    pub fn has_cap(&self, cap: &EnterpriseCap) -> bool {
        self.capabilities.contains(cap)
    }
    pub fn cap_count(&self) -> usize {
        self.capabilities.len()
    }
    pub fn is_expired(&self, current_tick: u64) -> bool {
        self.expires_tick.map(|t| current_tick > t).unwrap_or(false)
    }
    pub fn is_valid(&self, current_tick: u64) -> bool {
        !self.is_expired(current_tick)
    }
    pub fn is_enterprise_or_above(&self) -> bool {
        matches!(self.tier, LicenseTier::Enterprise | LicenseTier::GovCloud)
    }
}
