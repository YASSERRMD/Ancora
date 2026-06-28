use crate::attack::AttackOutcome;

#[test]
fn success() { assert_eq!(AttackOutcome::Success.to_string(), "SUCCESS"); }
#[test]
fn partial() { assert_eq!(AttackOutcome::PartialSuccess.to_string(), "PARTIAL_SUCCESS"); }
#[test]
fn failure() { assert_eq!(AttackOutcome::Failure.to_string(), "FAILURE"); }
#[test]
fn detected() { assert_eq!(AttackOutcome::Detected.to_string(), "DETECTED"); }
#[test]
fn blocked() { assert_eq!(AttackOutcome::Blocked.to_string(), "BLOCKED"); }
