use crate::config_push::*;
use crate::registration::DeviceId;

#[test]
fn test_config_push_applied() {
    let mut svc = ConfigPushService::new();
    let mut cfg = DeviceConfig::new(5);
    cfg.set("log_level", "debug");
    cfg.set("max_threads", "4");

    let id = DeviceId::new("dev-001");
    let record = svc.push(&id, &cfg);

    assert_eq!(record.config_version, 5);
    assert_eq!(record.status, PushStatus::Applied);
    assert_eq!(svc.applied_version(&id), Some(5));
}

#[test]
fn test_config_push_to_fleet() {
    let mut svc = ConfigPushService::new();
    let cfg = DeviceConfig::new(10);
    let ids: Vec<DeviceId> = (0..5).map(|i| DeviceId::new(format!("dev-{}", i))).collect();

    let records = svc.push_to_fleet(&ids, &cfg);
    assert_eq!(records.len(), 5);
    for r in &records {
        assert_eq!(r.status, PushStatus::Applied);
    }
}

#[test]
fn test_config_pending_devices() {
    let mut svc = ConfigPushService::new();
    let ids: Vec<DeviceId> = (0..4).map(|i| DeviceId::new(format!("dev-{}", i))).collect();

    // Push version 3 to first two devices
    let cfg_v3 = DeviceConfig::new(3);
    svc.push_to_fleet(&ids[..2], &cfg_v3);

    let pending = svc.pending_devices(3, &ids);
    assert_eq!(pending.len(), 2);
}
