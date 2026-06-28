use crate::maintenance::MaintenanceWindow;

#[test]
fn inactive_by_default() {
    let mw = MaintenanceWindow::new();
    assert!(!mw.is_active());
}

#[test]
fn enter_activates_window() {
    let mut mw = MaintenanceWindow::new();
    mw.enter(1000, "schema upgrade");
    assert!(mw.is_active());
    assert_eq!(mw.reason.as_deref(), Some("schema upgrade"));
}

#[test]
fn duration_increases_with_time() {
    let mut mw = MaintenanceWindow::new();
    mw.enter(100, "test");
    assert_eq!(mw.duration_secs(200), Some(100));
}

#[test]
fn exit_deactivates_window() {
    let mut mw = MaintenanceWindow::new();
    mw.enter(0, "test");
    mw.exit();
    assert!(!mw.is_active());
    assert!(mw.reason.is_none());
}
