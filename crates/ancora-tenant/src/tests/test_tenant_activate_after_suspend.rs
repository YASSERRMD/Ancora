use crate::Tenant;
#[test]
fn activate_after_suspend_restores_active() {
    let mut t = Tenant::new("t1", "A", 1);
    t.suspend();
    assert!(t.is_suspended());
    t.activate();
    assert!(t.is_active());
    assert!(!t.is_suspended());
}
