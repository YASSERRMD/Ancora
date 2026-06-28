use crate::{BootPolicy, MeasurementKind};
use std::collections::HashSet;
#[test]
fn required_kinds_met_when_all_present() {
    let p = BootPolicy::new("t1").require_kind(MeasurementKind::Kernel);
    let mut present = HashSet::new();
    present.insert("KERNEL".to_string());
    assert!(p.required_kinds_met(&present));
}
#[test]
fn required_kinds_not_met_when_missing() {
    let p = BootPolicy::new("t1").require_kind(MeasurementKind::Firmware);
    let present = HashSet::new();
    assert!(!p.required_kinds_met(&present));
}
