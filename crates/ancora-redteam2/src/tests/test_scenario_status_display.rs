use crate::scenario::ScenarioStatus;

#[test]
fn pending() { assert_eq!(ScenarioStatus::Pending.to_string(), "PENDING"); }
#[test]
fn running() { assert_eq!(ScenarioStatus::Running.to_string(), "RUNNING"); }
#[test]
fn completed() { assert_eq!(ScenarioStatus::Completed.to_string(), "COMPLETED"); }
#[test]
fn failed() { assert_eq!(ScenarioStatus::Failed.to_string(), "FAILED"); }
#[test]
fn aborted() { assert_eq!(ScenarioStatus::Aborted.to_string(), "ABORTED"); }
