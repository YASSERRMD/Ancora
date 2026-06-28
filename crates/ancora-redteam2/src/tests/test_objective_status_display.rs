use crate::objective::ObjectiveStatus;

#[test]
fn pending() { assert_eq!(ObjectiveStatus::Pending.to_string(), "PENDING"); }
#[test]
fn in_progress() { assert_eq!(ObjectiveStatus::InProgress.to_string(), "IN_PROGRESS"); }
#[test]
fn achieved() { assert_eq!(ObjectiveStatus::Achieved.to_string(), "ACHIEVED"); }
#[test]
fn failed() { assert_eq!(ObjectiveStatus::Failed.to_string(), "FAILED"); }
