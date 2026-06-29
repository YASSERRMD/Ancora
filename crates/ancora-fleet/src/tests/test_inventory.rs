use crate::inventory::*;
use crate::registration::DeviceId;

#[test]
fn test_inventory_accurate() {
    let mut inv = FleetInventory::new();
    let id = DeviceId::new("dev-001");
    let mut record = DeviceInventory::new(id.clone(), "edge-host-1")
        .with_os("Linux", "x86_64")
        .with_resources(8, 16384, 512);
    record.add_model("gpt-small-v1");

    inv.update(record);
    assert_eq!(inv.count(), 1);

    let entry = inv.get(&id).unwrap();
    assert_eq!(entry.os, "Linux");
    assert_eq!(entry.cpu_cores, 8);
    assert_eq!(entry.memory_mb, 16384);
    assert!(entry.has_model("gpt-small-v1"));
}

#[test]
fn test_inventory_summary() {
    let mut inv = FleetInventory::new();
    for i in 0..4 {
        let record = DeviceInventory::new(DeviceId::new(format!("dev-{}", i)), format!("host-{}", i))
            .with_resources(4, 8192, 256);
        inv.update(record);
    }
    let summary = inv.summary();
    assert_eq!(summary.total_devices, 4);
    assert_eq!(summary.total_cpu_cores, 16);
    assert_eq!(summary.total_memory_mb, 32768);
}

#[test]
fn test_inventory_devices_with_model() {
    let mut inv = FleetInventory::new();
    for i in 0..3 {
        let mut record = DeviceInventory::new(DeviceId::new(format!("dev-{}", i)), format!("h{}", i));
        if i < 2 {
            record.add_model("llm-tiny");
        }
        inv.update(record);
    }
    assert_eq!(inv.devices_with_model("llm-tiny").len(), 2);
    assert_eq!(inv.devices_with_model("llm-tiny").len(), 2);
}
