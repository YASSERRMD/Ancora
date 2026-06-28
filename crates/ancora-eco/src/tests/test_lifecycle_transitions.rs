use crate::lifecycle::{ExtensionLifecycle, LifecycleState};

#[test]
fn lifecycle_full_load_sequence() {
    let mut ext = ExtensionLifecycle::new("my-ext");
    assert_eq!(ext.state, LifecycleState::Registered);

    ext.transition(LifecycleState::Loading).unwrap();
    assert_eq!(ext.state, LifecycleState::Loading);

    ext.transition(LifecycleState::Active).unwrap();
    assert_eq!(ext.state, LifecycleState::Active);

    ext.transition(LifecycleState::Deprecated).unwrap();
    assert_eq!(ext.state, LifecycleState::Deprecated);

    ext.transition(LifecycleState::Unloading).unwrap();
    assert_eq!(ext.state, LifecycleState::Unloading);

    ext.transition(LifecycleState::Unloaded).unwrap();
    assert_eq!(ext.state, LifecycleState::Unloaded);
}

#[test]
fn lifecycle_can_fail_from_loading() {
    let mut ext = ExtensionLifecycle::new("bad-ext");
    ext.transition(LifecycleState::Loading).unwrap();
    ext.transition(LifecycleState::Failed("init error".to_string()))
        .unwrap();
    assert!(matches!(ext.state, LifecycleState::Failed(_)));
}

#[test]
fn invalid_transitions_are_rejected() {
    let mut ext = ExtensionLifecycle::new("test-ext");
    // Cannot jump from Registered to Deprecated.
    assert!(ext.transition(LifecycleState::Deprecated).is_err());
    // Cannot jump from Registered to Unloaded.
    assert!(ext.transition(LifecycleState::Unloaded).is_err());
    // State is unchanged.
    assert_eq!(ext.state, LifecycleState::Registered);
}

#[test]
fn lifecycle_id_preserved() {
    let ext = ExtensionLifecycle::new("identified-ext");
    assert_eq!(ext.id, "identified-ext");
}
