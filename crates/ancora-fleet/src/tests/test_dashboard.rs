use crate::dashboard::*;
use crate::registration::*;
use crate::inventory::*;
use crate::telemetry::*;
use std::collections::HashMap;

fn setup_registry_with_devices(count: usize) -> DeviceRegistry {
    let mut registry = DeviceRegistry::new();
    for i in 0..count {
        let req = RegistrationRequest {
            device_id: DeviceId::new(format!("dev-{}", i)),
            name: format!("Node {}", i),
            fingerprint: format!("fp-{}", i),
            metadata: HashMap::new(),
        };
        registry.register(req);
    }
    registry
}

#[test]
fn test_dashboard_json_valid() {
    let registry = setup_registry_with_devices(3);
    let mut inventory = FleetInventory::new();
    for i in 0..3 {
        let record = DeviceInventory::new(DeviceId::new(format!("dev-{}", i)), format!("host-{}", i))
            .with_resources(4, 8192, 100);
        inventory.update(record);
    }
    let telemetry = FleetTelemetry::new();
    let dashboard = build_dashboard(&registry, &inventory, &telemetry);

    assert_eq!(dashboard.total_devices, 3);
    assert_eq!(dashboard.active_devices, 3);

    let json = dashboard_to_json(&dashboard).unwrap();
    assert!(json.contains("total_devices"));
    assert!(json.contains("active_devices"));
}

#[test]
fn test_dashboard_unhealthy_count() {
    let registry = setup_registry_with_devices(2);
    let inventory = FleetInventory::new();
    let mut telemetry = FleetTelemetry::new();

    // One healthy, one unhealthy
    let s_ok = HealthSample::new(DeviceId::new("dev-0")).with_metrics(20.0, 30.0, 40.0, 1000);
    let mut s_bad = HealthSample::new(DeviceId::new("dev-1")).with_metrics(98.0, 95.0, 50.0, 50);
    s_bad.error_count = 2;

    telemetry.ingest(s_ok);
    telemetry.ingest(s_bad);

    let dashboard = build_dashboard(&registry, &inventory, &telemetry);
    assert_eq!(dashboard.healthy_devices + dashboard.unhealthy_devices, 2);
}
