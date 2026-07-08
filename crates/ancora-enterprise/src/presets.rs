use crate::checkpoint::{CheckStatus, EnterpriseCheckpoint, HealthCheck};
use crate::feature::{FeatureFlag, FeatureRegistry, FeatureState};
use crate::license::{EnterpriseCap, EnterpriseLicense, LicenseTier};
use crate::posture::{DomainScore, SecurityPosture};

pub fn enterprise_license(
    id: impl Into<String>,
    tenant_id: impl Into<String>,
    tick: u64,
) -> EnterpriseLicense {
    EnterpriseLicense::new(id, tenant_id, LicenseTier::Enterprise, 1000, 10, tick)
        .with_cap(EnterpriseCap::Hsm)
        .with_cap(EnterpriseCap::AirGap)
        .with_cap(EnterpriseCap::RedTeamSim)
        .with_cap(EnterpriseCap::PentestAutomation)
        .with_cap(EnterpriseCap::AdvancedCompliance)
        .with_cap(EnterpriseCap::SsoIntegration)
        .with_cap(EnterpriseCap::AuditExport)
        .with_cap(EnterpriseCap::MultiRegion)
        .with_cap(EnterpriseCap::CustomRoles)
        .with_cap(EnterpriseCap::ThreatIntelFeed)
}

pub fn community_license(
    id: impl Into<String>,
    tenant_id: impl Into<String>,
    tick: u64,
) -> EnterpriseLicense {
    EnterpriseLicense::new(id, tenant_id, LicenseTier::Community, 10, 1, tick)
}

pub fn default_feature_registry() -> FeatureRegistry {
    let mut r = FeatureRegistry::new();
    r.register(FeatureFlag::new(
        "hsm-integration",
        FeatureState::Enabled,
        "HSM hardware key management",
    ));
    r.register(FeatureFlag::new(
        "airgap-transfer",
        FeatureState::Enabled,
        "Air-gapped data transfer control",
    ));
    r.register(FeatureFlag::new(
        "red-team-sim",
        FeatureState::Enabled,
        "Red team scenario simulation",
    ));
    r.register(FeatureFlag::new(
        "pentest-automation",
        FeatureState::Enabled,
        "Automated penetration testing",
    ));
    r.register(FeatureFlag::new(
        "advanced-audit",
        FeatureState::Enabled,
        "Advanced audit trail with export",
    ));
    r.register(FeatureFlag::new(
        "threat-intel",
        FeatureState::BetaOnly,
        "Threat intelligence feed integration",
    ));
    r.register(FeatureFlag::new(
        "quantum-safe-keys",
        FeatureState::Disabled,
        "Quantum-safe cryptography",
    ));
    r
}

pub fn standard_checkpoint(tick: u64) -> EnterpriseCheckpoint {
    let mut cp = EnterpriseCheckpoint::new(tick);
    cp.add(HealthCheck::new(
        "chk-1",
        "License validity",
        "licensing",
        CheckStatus::Pass,
        "License active",
        tick,
    ));
    cp.add(HealthCheck::new(
        "chk-2",
        "HSM connectivity",
        "hsm",
        CheckStatus::Pass,
        "HSM slots operational",
        tick,
    ));
    cp.add(HealthCheck::new(
        "chk-3",
        "Audit log integrity",
        "audit",
        CheckStatus::Pass,
        "No gaps detected",
        tick,
    ));
    cp.add(HealthCheck::new(
        "chk-4",
        "Air-gap policy",
        "airgap",
        CheckStatus::Pass,
        "Policy enforced",
        tick,
    ));
    cp.add(HealthCheck::new(
        "chk-5",
        "Incident queue",
        "incidents",
        CheckStatus::Warn,
        "2 open incidents",
        tick,
    ));
    cp
}

pub fn healthy_posture(tenant_id: impl Into<String>, tick: u64) -> SecurityPosture {
    let mut p = SecurityPosture::new(tenant_id, tick);
    p.add_domain(DomainScore::new("hsm", 90, 1, 0));
    p.add_domain(DomainScore::new("airgap", 85, 2, 0));
    p.add_domain(DomainScore::new("pentest", 75, 5, 0));
    p.add_domain(DomainScore::new("redteam", 80, 3, 0));
    p
}
