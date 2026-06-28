use crate::{Measurement, MeasurementKind};
#[test]
fn with_metadata_stores_value() {
    let m = Measurement::new("m1", MeasurementKind::Application, "app", "d1", 0)
        .with_metadata("vendor", "acme");
    assert_eq!(m.metadata.get("vendor").map(String::as_str), Some("acme"));
}
