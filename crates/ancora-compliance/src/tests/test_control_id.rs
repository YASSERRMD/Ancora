use crate::ControlId;
#[test]
fn control_id_display() {
    let id = ControlId::new("CC6.1");
    assert_eq!(format!("{}", id), "CC6.1");
    assert_eq!(id.0, "CC6.1");
}
