use crate::decommission::*;
use crate::registration::*;
use std::collections::HashMap;

fn make_registry_with_device(id: &str) -> DeviceRegistry {
    let mut registry = DeviceRegistry::new();
    let req = RegistrationRequest {
        device_id: DeviceId::new(id),
        name: format!("Device {}", id),
        fingerprint: format!("fp-{}", id),
        metadata: HashMap::new(),
    };
    registry.register(req);
    registry
}

#[test]
fn test_decommission_revokes_device() {
    let mut registry = make_registry_with_device("dev-001");
    let mut svc = DecommissionService::new();

    let id = DeviceId::new("dev-001");
    let result = svc.decommission(&mut registry, &id, DecommissionReason::EndOfLife);

    assert!(result.is_ok());
    let record = result.unwrap();
    assert!(record.revoked);
    assert!(svc.is_decommissioned(&id));
}

#[test]
fn test_decommission_non_existent_device() {
    let mut registry = DeviceRegistry::new();
    let mut svc = DecommissionService::new();
    let id = DeviceId::new("ghost-device");
    let result = svc.decommission(&mut registry, &id, DecommissionReason::Other("test".into()));
    assert!(result.is_err());
}

#[test]
fn test_decommissioned_device_not_active() {
    let mut registry = make_registry_with_device("dev-002");
    let mut svc = DecommissionService::new();

    let id = DeviceId::new("dev-002");
    svc.decommission(&mut registry, &id, DecommissionReason::Replaced)
        .unwrap();

    let identity = registry.get(&id).unwrap();
    assert!(!identity.is_active());
    assert_eq!(identity.status, RegistrationStatus::Decommissioned);
}
