use crate::slot::{HsmSlot, SlotManager};

#[test]
fn slot_manager_add_and_get() {
    let mut mgr = SlotManager::new();
    let s = HsmSlot::new(0, "S0", "SoftHSM");
    mgr.add_slot(s);
    assert!(mgr.get(0).is_some());
    assert_eq!(mgr.count(), 1);
}

#[test]
fn slot_manager_slots_with_token() {
    let mut mgr = SlotManager::new();
    let mut s0 = HsmSlot::new(0, "S0", "SoftHSM");
    s0.insert_token();
    let s1 = HsmSlot::new(1, "S1", "SoftHSM");
    mgr.add_slot(s0);
    mgr.add_slot(s1);
    assert_eq!(mgr.slots_with_token().len(), 1);
}

#[test]
fn slot_manager_get_mut() {
    let mut mgr = SlotManager::new();
    mgr.add_slot(HsmSlot::new(0, "S0", "SoftHSM"));
    let s = mgr.get_mut(0).unwrap();
    s.insert_token();
    assert!(mgr.get(0).unwrap().has_token());
}

#[test]
fn slot_manager_all() {
    let mut mgr = SlotManager::new();
    mgr.add_slot(HsmSlot::new(0, "S0", "SoftHSM"));
    mgr.add_slot(HsmSlot::new(1, "S1", "SoftHSM"));
    assert_eq!(mgr.all().len(), 2);
}
