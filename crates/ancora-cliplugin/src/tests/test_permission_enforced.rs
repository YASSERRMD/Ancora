use crate::interface::PluginError;
use crate::permissions::{PermissionEnforcer, PermissionGrant, PermissionScope};

#[test]
fn test_granted_permission_passes() {
    let mut enforcer = PermissionEnforcer::new();
    enforcer.grant("my.plugin", PermissionScope::FsRead);

    let result = enforcer.check("my.plugin", &PermissionScope::FsRead);
    assert!(result.is_ok(), "granted permission should pass check");
}

#[test]
fn test_denied_permission_returns_error() {
    let enforcer = PermissionEnforcer::new();
    let result = enforcer.check("my.plugin", &PermissionScope::Network);

    assert!(
        matches!(result, Err(PluginError::PermissionDenied(_))),
        "missing permission should return PermissionDenied"
    );
}

#[test]
fn test_revoke_removes_permission() {
    let mut enforcer = PermissionEnforcer::new();
    enforcer.grant("my.plugin", PermissionScope::FsWrite);
    enforcer.revoke("my.plugin", &PermissionScope::FsWrite);

    let result = enforcer.check("my.plugin", &PermissionScope::FsWrite);
    assert!(result.is_err(), "revoked permission should not pass");
}

#[test]
fn test_all_grant_allows_everything() {
    let mut enforcer = PermissionEnforcer::new();
    enforcer.set_grant("my.plugin", PermissionGrant::all());

    let scopes = [
        PermissionScope::FsRead,
        PermissionScope::FsWrite,
        PermissionScope::Network,
        PermissionScope::Exec,
        PermissionScope::EnvRead,
        PermissionScope::ConfigWrite,
    ];

    for scope in &scopes {
        assert!(
            enforcer.check("my.plugin", scope).is_ok(),
            "all-grant should allow {:?}",
            scope
        );
    }
}

#[test]
fn test_different_plugins_have_independent_grants() {
    let mut enforcer = PermissionEnforcer::new();
    enforcer.grant("plug.a", PermissionScope::Network);

    // plug.b has no grants
    assert!(enforcer.check("plug.b", &PermissionScope::Network).is_err());
    assert!(enforcer.check("plug.a", &PermissionScope::Network).is_ok());
}

#[test]
fn test_custom_scope_is_recognized() {
    let mut enforcer = PermissionEnforcer::new();
    let custom = PermissionScope::Custom("read:secrets".to_string());
    enforcer.grant("my.plugin", custom.clone());

    assert!(enforcer.check("my.plugin", &custom).is_ok());
    assert_eq!(custom.as_str(), "read:secrets");
}

#[test]
fn test_permission_scope_from_str() {
    assert_eq!(PermissionScope::from_str("fs:read"), Some(PermissionScope::FsRead));
    assert_eq!(PermissionScope::from_str("network"), Some(PermissionScope::Network));
    assert_eq!(
        PermissionScope::from_str("custom:thing"),
        Some(PermissionScope::Custom("custom:thing".to_string()))
    );
}
