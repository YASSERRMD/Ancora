use crate::presets::default_slot;

#[test]
fn default_slot_has_token() {
    let s = default_slot();
    assert!(s.has_token());
}

#[test]
fn default_slot_id_is_zero() {
    let s = default_slot();
    assert_eq!(s.id, 0);
}

#[test]
fn default_slot_has_label() {
    let s = default_slot();
    assert!(!s.label.is_empty());
}
