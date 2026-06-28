use crate::{BootChain, BootPolicy, Measurement, MeasurementKind};
use crate::validator::{ChainIssue, ChainValidator};
#[test]
fn empty_chain_produces_empty_chain_issue() {
    let chain = BootChain::new("t1", "n1");
    let policy = BootPolicy::new("t1").allow_unknown();
    let issues = ChainValidator::validate(&chain, &policy);
    assert!(issues.contains(&ChainIssue::EmptyChain));
}
#[test]
fn missing_required_kind_reported() {
    let policy = BootPolicy::new("t1").require_kind(MeasurementKind::Firmware).allow_unknown();
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "k", "d", 0));
    let issues = ChainValidator::validate(&chain, &policy);
    assert!(issues.iter().any(|i| matches!(i, ChainIssue::MissingRequiredKind(_))));
}
#[test]
fn valid_chain_has_no_issues() {
    let policy = BootPolicy::new("t1").allow_unknown();
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "k", "d", 0));
    assert!(ChainValidator::is_valid(&chain, &policy));
}
