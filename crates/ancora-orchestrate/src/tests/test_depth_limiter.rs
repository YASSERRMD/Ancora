use crate::depth_limiter::DepthLimiter;

#[test]
fn enter_increments_depth() {
    let mut lim = DepthLimiter::new(5);
    lim.enter().unwrap();
    assert_eq!(lim.depth(), 1);
}

#[test]
fn exceed_max_depth_errors() {
    let mut lim = DepthLimiter::new(2);
    lim.enter().unwrap();
    lim.enter().unwrap();
    assert!(lim.enter().is_err());
}

#[test]
fn exit_decrements_depth() {
    let mut lim = DepthLimiter::new(5);
    lim.enter().unwrap();
    lim.exit();
    assert_eq!(lim.depth(), 0);
}
