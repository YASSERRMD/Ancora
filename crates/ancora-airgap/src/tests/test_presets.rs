use crate::media::MediaType;
use crate::policy::PolicyVerdict;
use crate::presets::{data_import_procedure, restricted_zone, standard_airgap_policy, strict_airgap_policy, top_secret_zone};
use crate::transfer::{TransferDirection, TransferRequest};

#[test]
fn strict_policy_blocks_bluetooth() {
    let policy = strict_airgap_policy("t1");
    let req = TransferRequest::new("r1", "t1", "alice", MediaType::Bluetooth, TransferDirection::Inbound, "", 1);
    assert!(matches!(policy.evaluate(&req), PolicyVerdict::Deny(_)));
}

#[test]
fn standard_policy_allows_printed_documents() {
    let policy = standard_airgap_policy("t1");
    let req = TransferRequest::new("r1", "t1", "alice", MediaType::PrintedDocument, TransferDirection::Inbound, "", 1);
    assert_eq!(policy.evaluate(&req), PolicyVerdict::Allow);
}

#[test]
fn restricted_zone_is_restricted() {
    let z = restricted_zone("t1");
    assert!(z.is_restricted());
}

#[test]
fn top_secret_zone_is_restricted() {
    let z = top_secret_zone("t1");
    assert!(z.is_restricted());
}

#[test]
fn data_import_procedure_has_steps() {
    let p = data_import_procedure("t1");
    assert_eq!(p.step_count(), 5);
    assert_eq!(p.completed_count(), 0);
}
