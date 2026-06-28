use crate::license::EnterpriseCap;

#[test]
fn hsm() { assert_eq!(EnterpriseCap::Hsm.to_string(), "HSM"); }
#[test]
fn airgap() { assert_eq!(EnterpriseCap::AirGap.to_string(), "AIR_GAP"); }
#[test]
fn redteam() { assert_eq!(EnterpriseCap::RedTeamSim.to_string(), "RED_TEAM_SIM"); }
#[test]
fn pentest() { assert_eq!(EnterpriseCap::PentestAutomation.to_string(), "PENTEST_AUTOMATION"); }
#[test]
fn compliance() { assert_eq!(EnterpriseCap::AdvancedCompliance.to_string(), "ADVANCED_COMPLIANCE"); }
#[test]
fn sso() { assert_eq!(EnterpriseCap::SsoIntegration.to_string(), "SSO_INTEGRATION"); }
#[test]
fn audit_export() { assert_eq!(EnterpriseCap::AuditExport.to_string(), "AUDIT_EXPORT"); }
#[test]
fn multi_region() { assert_eq!(EnterpriseCap::MultiRegion.to_string(), "MULTI_REGION"); }
#[test]
fn custom_roles() { assert_eq!(EnterpriseCap::CustomRoles.to_string(), "CUSTOM_ROLES"); }
#[test]
fn threat_intel() { assert_eq!(EnterpriseCap::ThreatIntelFeed.to_string(), "THREAT_INTEL_FEED"); }
