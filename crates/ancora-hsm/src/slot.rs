use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlotState {
    Empty,
    TokenPresent,
    TokenAbsent,
}

impl fmt::Display for SlotState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SlotState::Empty => "EMPTY",
            SlotState::TokenPresent => "TOKEN_PRESENT",
            SlotState::TokenAbsent => "TOKEN_ABSENT",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct HsmSlot {
    pub id: u32,
    pub label: String,
    pub state: SlotState,
    pub manufacturer: String,
    pub flags: HashMap<String, bool>,
}

impl HsmSlot {
    pub fn new(id: u32, label: impl Into<String>, manufacturer: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            state: SlotState::Empty,
            manufacturer: manufacturer.into(),
            flags: HashMap::new(),
        }
    }

    pub fn insert_token(&mut self) { self.state = SlotState::TokenPresent; }
    pub fn remove_token(&mut self) { self.state = SlotState::TokenAbsent; }
    pub fn has_token(&self) -> bool { self.state == SlotState::TokenPresent }
    pub fn set_flag(&mut self, key: impl Into<String>, value: bool) { self.flags.insert(key.into(), value); }
}

pub struct SlotManager {
    slots: Vec<HsmSlot>,
}

impl SlotManager {
    pub fn new() -> Self { Self { slots: Vec::new() } }
    pub fn add_slot(&mut self, slot: HsmSlot) { self.slots.push(slot); }
    pub fn get(&self, id: u32) -> Option<&HsmSlot> { self.slots.iter().find(|s| s.id == id) }
    pub fn get_mut(&mut self, id: u32) -> Option<&mut HsmSlot> { self.slots.iter_mut().find(|s| s.id == id) }
    pub fn slots_with_token(&self) -> Vec<&HsmSlot> { self.slots.iter().filter(|s| s.has_token()).collect() }
    pub fn count(&self) -> usize { self.slots.len() }
    pub fn all(&self) -> &[HsmSlot] { &self.slots }
}
