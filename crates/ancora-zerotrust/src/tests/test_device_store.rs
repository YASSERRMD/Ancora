use crate::device::{DevicePosture, DeviceStore};

#[test]
fn device_store_upsert_get() {
    let mut store = DeviceStore::new();
    store.upsert(DevicePosture::new("d1", "t1", 1));
    assert!(store.get("d1").is_some());
    assert_eq!(store.count(), 1);
}

#[test]
fn device_store_for_tenant() {
    let mut store = DeviceStore::new();
    store.upsert(DevicePosture::new("d1", "t1", 1));
    store.upsert(DevicePosture::new("d2", "t2", 1));
    assert_eq!(store.for_tenant("t1").len(), 1);
}

#[test]
fn device_store_trusted() {
    let mut store = DeviceStore::new();
    let mut d = DevicePosture::new("d1", "t1", 1);
    d.os_up_to_date = true;
    d.antivirus_active = true;
    d.disk_encrypted = true;
    d.compute_trust();
    store.upsert(d);
    store.upsert(DevicePosture::new("d2", "t1", 1));
    assert_eq!(store.trusted().len(), 1);
}
