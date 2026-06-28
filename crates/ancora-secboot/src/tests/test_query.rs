use crate::{Measurement, MeasurementKind};
use crate::query::MeasurementQuery;
#[test]
fn query_by_kind_filters_correctly() {
    let m1 = Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "d1", 0);
    let m2 = Measurement::new("m2", MeasurementKind::Firmware, "fw", "d2", 0);
    let measurements = vec![m1, m2];
    let q = MeasurementQuery::new().kind(MeasurementKind::Kernel);
    let result = q.run(measurements.iter());
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "vmlinuz");
}
#[test]
fn query_by_name_contains_filters() {
    let m1 = Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz-6.1", "d1", 0);
    let m2 = Measurement::new("m2", MeasurementKind::Firmware, "uefi.bin", "d2", 0);
    let measurements = vec![m1, m2];
    let q = MeasurementQuery::new().name_contains("vmlinuz");
    let result = q.run(measurements.iter());
    assert_eq!(result.len(), 1);
}
