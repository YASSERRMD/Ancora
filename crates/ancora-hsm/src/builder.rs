use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use crate::slot::HsmSlot;

pub struct HsmKeyBuilder {
    handle: u64,
    slot_id: u32,
    label: String,
    algorithm: HsmKeyAlgorithm,
    class: KeyClass,
    tick: u64,
    extractable: bool,
    sensitive: bool,
}

impl HsmKeyBuilder {
    pub fn new(
        handle: u64,
        slot_id: u32,
        label: impl Into<String>,
        algorithm: HsmKeyAlgorithm,
    ) -> Self {
        Self {
            handle,
            slot_id,
            label: label.into(),
            algorithm,
            class: KeyClass::SecretKey,
            tick: 0,
            extractable: false,
            sensitive: true,
        }
    }
    pub fn class(mut self, c: KeyClass) -> Self {
        self.class = c;
        self
    }
    pub fn tick(mut self, t: u64) -> Self {
        self.tick = t;
        self
    }
    pub fn extractable(mut self) -> Self {
        self.extractable = true;
        self
    }
    pub fn not_sensitive(mut self) -> Self {
        self.sensitive = false;
        self
    }
    pub fn build(self) -> HsmKey {
        let mut k = HsmKey::new(
            self.handle,
            self.slot_id,
            self.label,
            self.algorithm,
            self.class,
            self.tick,
        );
        k.extractable = self.extractable;
        k.sensitive = self.sensitive;
        k
    }
}

pub struct SlotBuilder {
    id: u32,
    label: String,
    manufacturer: String,
    with_token: bool,
}

impl SlotBuilder {
    pub fn new(id: u32, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            manufacturer: "SoftHSM".into(),
            with_token: false,
        }
    }
    pub fn manufacturer(mut self, m: impl Into<String>) -> Self {
        self.manufacturer = m.into();
        self
    }
    pub fn with_token(mut self) -> Self {
        self.with_token = true;
        self
    }
    pub fn build(self) -> HsmSlot {
        let mut slot = HsmSlot::new(self.id, self.label, self.manufacturer);
        if self.with_token {
            slot.insert_token();
        }
        slot
    }
}
