use crate::scenario::ScenarioKind;

#[test]
fn priv_esc() {
    assert_eq!(
        ScenarioKind::PrivilegeEscalation.to_string(),
        "PRIVILEGE_ESCALATION"
    );
}
#[test]
fn lateral() {
    assert_eq!(
        ScenarioKind::LateralMovement.to_string(),
        "LATERAL_MOVEMENT"
    );
}
#[test]
fn exfil() {
    assert_eq!(
        ScenarioKind::DataExfiltration.to_string(),
        "DATA_EXFILTRATION"
    );
}
#[test]
fn cred() {
    assert_eq!(
        ScenarioKind::CredentialHarvesting.to_string(),
        "CREDENTIAL_HARVESTING"
    );
}
#[test]
fn persistence() {
    assert_eq!(
        ScenarioKind::PersistenceMechanism.to_string(),
        "PERSISTENCE_MECHANISM"
    );
}
#[test]
fn evasion() {
    assert_eq!(ScenarioKind::DefenseEvasion.to_string(), "DEFENSE_EVASION");
}
#[test]
fn c2() {
    assert_eq!(
        ScenarioKind::CommandAndControl.to_string(),
        "COMMAND_AND_CONTROL"
    );
}
#[test]
fn initial_access() {
    assert_eq!(ScenarioKind::InitialAccess.to_string(), "INITIAL_ACCESS");
}
#[test]
fn recon() {
    assert_eq!(
        ScenarioKind::CollectionAndRecon.to_string(),
        "COLLECTION_AND_RECON"
    );
}
#[test]
fn impact() {
    assert_eq!(
        ScenarioKind::ImpactAndDisruption.to_string(),
        "IMPACT_AND_DISRUPTION"
    );
}
