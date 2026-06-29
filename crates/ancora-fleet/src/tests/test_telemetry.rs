use crate::telemetry::*;
use crate::registration::DeviceId;

#[test]
fn test_health_telemetry_received() {
    let mut telem = FleetTelemetry::new();
    let id = DeviceId::new("dev-001");

    let sample = HealthSample::new(id.clone())
        .with_metrics(30.0, 45.0, 60.0, 3600);

    telem.ingest(sample);
    let latest = telem.latest_for(&id).unwrap();
    assert_eq!(latest.cpu_percent, 30.0);
    assert_eq!(latest.memory_percent, 45.0);
    assert!(latest.is_healthy());
}

#[test]
fn test_unhealthy_device_detected() {
    let mut telem = FleetTelemetry::new();
    let id = DeviceId::new("dev-bad");

    let mut sample = HealthSample::new(id.clone())
        .with_metrics(95.0, 80.0, 70.0, 100);
    sample.error_count = 3;

    telem.ingest(sample);
    assert!(!telem.latest_for(&id).unwrap().is_healthy());
    let unhealthy = telem.unhealthy_devices();
    assert_eq!(unhealthy.len(), 1);
}

#[test]
fn test_alert_level_critical() {
    let mut telem = FleetTelemetry::new();
    let id = DeviceId::new("crit-dev");

    let sample = HealthSample::new(id.clone())
        .with_metrics(98.0, 97.0, 50.0, 50);

    telem.ingest(sample);
    assert_eq!(telem.alert_level(&id), AlertLevel::Critical);
}

#[test]
fn test_telemetry_history() {
    let mut telem = FleetTelemetry::new();
    let id = DeviceId::new("dev-hist");

    for i in 0..5 {
        let s = HealthSample::new(id.clone())
            .with_metrics(i as f32 * 10.0, 20.0, 30.0, i as u64 * 60);
        telem.ingest(s);
    }
    assert_eq!(telem.history_for(&id).len(), 5);
}
