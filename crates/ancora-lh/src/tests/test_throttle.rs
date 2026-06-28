use crate::throttle::Throttle;

#[test]
fn throttle_allows_ops_within_limit() {
    let mut t = Throttle::new(3);
    assert!(t.try_op(1).is_ok());
    assert!(t.try_op(1).is_ok());
    assert!(t.try_op(1).is_ok());
}

#[test]
fn throttle_blocks_over_limit() {
    let mut t = Throttle::new(2);
    t.try_op(1).unwrap();
    t.try_op(1).unwrap();
    assert!(t.try_op(1).is_err());
}

#[test]
fn throttle_resets_on_new_tick() {
    let mut t = Throttle::new(1);
    t.try_op(1).unwrap();
    assert!(t.try_op(1).is_err());
    assert!(t.try_op(2).is_ok());
}

#[test]
fn background_run_throttled() {
    let mut t = Throttle::new(1);
    assert!(t.try_op(5).is_ok());
    let result = t.try_op(5);
    assert!(result.is_err());
}
