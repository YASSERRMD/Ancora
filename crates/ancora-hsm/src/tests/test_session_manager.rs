use crate::session::SessionManager;

#[test]
fn session_manager_open_and_close() {
    let mut mgr = SessionManager::new();
    let id = mgr.open_session(0, true, 1);
    assert!(mgr.get(id).is_some());
    mgr.close_session(id);
    assert_eq!(mgr.active().len(), 0);
}

#[test]
fn session_manager_count() {
    let mut mgr = SessionManager::new();
    mgr.open_session(0, true, 1);
    mgr.open_session(0, false, 2);
    assert_eq!(mgr.count(), 2);
}

#[test]
fn session_manager_active() {
    let mut mgr = SessionManager::new();
    let id = mgr.open_session(0, true, 1);
    assert_eq!(mgr.active().len(), 1);
    mgr.close_session(id);
    assert_eq!(mgr.active().len(), 0);
}

#[test]
fn session_manager_get_mut() {
    let mut mgr = SessionManager::new();
    let id = mgr.open_session(0, true, 1);
    mgr.get_mut(id).unwrap().login();
    assert!(mgr.get(id).unwrap().is_logged_in());
}
