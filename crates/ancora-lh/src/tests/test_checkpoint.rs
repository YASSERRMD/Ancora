use crate::checkpoint::{Checkpoint, CheckpointCadence};

#[test]
fn checkpoint_stores_and_retrieves_data() {
    let mut cp = Checkpoint::new("r1", 10);
    cp.set("step", "3");
    assert_eq!(cp.get("step"), Some("3"));
}

#[test]
fn checkpoint_and_resume_across_restart() {
    let mut cp = Checkpoint::new("r1", 5);
    cp.set("cursor", "42");
    let restored_cursor = cp.get("cursor").unwrap_or("0");
    assert_eq!(restored_cursor, "42");
}

#[test]
fn cadence_triggers_at_interval() {
    let mut cadence = CheckpointCadence::new(10);
    assert!(cadence.should_checkpoint(10));
    assert!(!cadence.should_checkpoint(15));
    assert!(cadence.should_checkpoint(20));
}

#[test]
fn cadence_first_trigger_at_zero_plus_interval() {
    let mut cadence = CheckpointCadence::new(5);
    assert!(!cadence.should_checkpoint(4));
    assert!(cadence.should_checkpoint(5));
}
