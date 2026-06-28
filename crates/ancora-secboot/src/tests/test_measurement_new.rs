use crate::{Measurement, MeasurementKind};
#[test]
fn new_measurement_stores_fields() {
    let m = Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "abc123", 10);
    assert_eq!(m.id, "m1");
    assert_eq!(m.name, "vmlinuz");
    assert_eq!(m.digest, "abc123");
    assert_eq!(m.tick, 10);
}
