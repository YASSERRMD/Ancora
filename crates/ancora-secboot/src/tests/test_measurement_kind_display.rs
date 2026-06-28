use crate::MeasurementKind;
#[test]
fn kind_display() {
    assert_eq!(format!("{}", MeasurementKind::Firmware), "FIRMWARE");
    assert_eq!(format!("{}", MeasurementKind::Kernel), "KERNEL");
    assert_eq!(format!("{}", MeasurementKind::Application), "APPLICATION");
}
