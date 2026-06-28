use crate::permission::PermissionScope;
use crate::spec::EffectClass;

#[test]
fn read_only_scope_allows_read() {
    let scope = PermissionScope::read_only();
    assert!(scope.check(&EffectClass::ReadOnly).is_ok());
}

#[test]
fn read_only_scope_blocks_write() {
    let scope = PermissionScope::read_only();
    assert!(scope.check(&EffectClass::WriteExternal).is_err());
}

#[test]
fn local_write_scope_allows_read_and_write() {
    let scope = PermissionScope::local_write();
    assert!(scope.check(&EffectClass::ReadOnly).is_ok());
    assert!(scope.check(&EffectClass::WriteLocal).is_ok());
}

#[test]
fn local_write_scope_blocks_external() {
    let scope = PermissionScope::local_write();
    assert!(scope.check(&EffectClass::WriteExternal).is_err());
}
