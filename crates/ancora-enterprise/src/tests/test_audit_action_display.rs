use crate::audit::EnterpriseAction;

#[test]
fn license_issued() { assert_eq!(EnterpriseAction::LicenseIssued.to_string(), "LICENSE_ISSUED"); }
#[test]
fn license_expired() { assert_eq!(EnterpriseAction::LicenseExpired.to_string(), "LICENSE_EXPIRED"); }
#[test]
fn feature_enabled() { assert_eq!(EnterpriseAction::FeatureEnabled.to_string(), "FEATURE_ENABLED"); }
#[test]
fn feature_disabled() { assert_eq!(EnterpriseAction::FeatureDisabled.to_string(), "FEATURE_DISABLED"); }
#[test]
fn incident_opened() { assert_eq!(EnterpriseAction::IncidentOpened.to_string(), "INCIDENT_OPENED"); }
#[test]
fn incident_resolved() { assert_eq!(EnterpriseAction::IncidentResolved.to_string(), "INCIDENT_RESOLVED"); }
#[test]
fn checkpoint_run() { assert_eq!(EnterpriseAction::CheckpointRun.to_string(), "CHECKPOINT_RUN"); }
#[test]
fn posture_assessed() { assert_eq!(EnterpriseAction::PostureAssessed.to_string(), "POSTURE_ASSESSED"); }
#[test]
fn report_generated() { assert_eq!(EnterpriseAction::ReportGenerated.to_string(), "REPORT_GENERATED"); }
