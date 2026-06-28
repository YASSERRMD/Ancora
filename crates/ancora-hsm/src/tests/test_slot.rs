use crate::slot::{HsmSlot, SlotState};

#[test]
fn slot_new_empty() {
    let s = HsmSlot::new(0, "Slot 0", "SoftHSM");
    assert_eq!(s.state, SlotState::Empty);
    assert!(!s.has_token());
}

#[test]
fn slot_insert_token() {
    let mut s = HsmSlot::new(0, "Slot 0", "SoftHSM");
    s.insert_token();
    assert!(s.has_token());
    assert_eq!(s.state, SlotState::TokenPresent);
}

#[test]
fn slot_remove_token() {
    let mut s = HsmSlot::new(0, "Slot 0", "SoftHSM");
    s.insert_token();
    s.remove_token();
    assert_eq!(s.state, SlotState::TokenAbsent);
    assert!(!s.has_token());
}

#[test]
fn slot_set_flag() {
    let mut s = HsmSlot::new(0, "Slot 0", "SoftHSM");
    s.set_flag("rw", true);
    assert_eq!(s.flags.get("rw"), Some(&true));
}
