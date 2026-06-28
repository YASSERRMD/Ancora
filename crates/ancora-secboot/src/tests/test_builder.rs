use crate::{MeasurementBuilder, MeasurementKind};
#[test]
fn builder_defaults_to_application_kind() {
    let m = MeasurementBuilder::new("m1", "app").build();
    assert_eq!(m.kind, MeasurementKind::Application);
    assert!(m.digest.is_empty());
}
#[test]
fn builder_sets_all_fields() {
    let m = MeasurementBuilder::new("m1", "vmlinuz")
        .kind(MeasurementKind::Kernel)
        .digest("deadbeef")
        .tick(100)
        .build();
    assert_eq!(m.kind, MeasurementKind::Kernel);
    assert_eq!(m.digest, "deadbeef");
    assert_eq!(m.tick, 100);
}
