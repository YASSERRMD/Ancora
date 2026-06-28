use crate::deadline::Deadline;

#[test]
fn deadline_enforced_past_tick() {
    let d = Deadline::new("r1", 100);
    assert!(d.check(101).is_err());
}

#[test]
fn deadline_passes_before_tick() {
    let d = Deadline::new("r1", 100);
    assert!(d.check(50).is_ok());
}

#[test]
fn deadline_at_exact_tick_passes() {
    let d = Deadline::new("r1", 100);
    assert!(d.check(100).is_ok());
}

#[test]
fn remaining_ticks_computed_correctly() {
    let d = Deadline::new("r1", 200);
    assert_eq!(d.remaining_ticks(150), 50);
    assert_eq!(d.remaining_ticks(250), 0);
}
