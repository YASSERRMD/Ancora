use crate::{Measurement, MeasurementKind};
#[test]
fn matches_digest_returns_correct_result() {
    let m = Measurement::new("m1", MeasurementKind::Firmware, "fw", "dead", 0);
    assert!(m.matches_digest("dead"));
    assert!(!m.matches_digest("beef"));
}
