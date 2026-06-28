use crate::key::HsmKeyAlgorithm;
use crate::mock::SoftHsm;
use crate::policy::HsmPolicy;
use crate::slot::HsmSlot;

pub fn aes256_key(hsm: &mut SoftHsm, slot_id: u32, tick: u64) -> u64 {
    hsm.generate_key(slot_id, "aes256-default", HsmKeyAlgorithm::Aes256, tick)
}

pub fn ed25519_signing_key(hsm: &mut SoftHsm, slot_id: u32, tick: u64) -> u64 {
    hsm.generate_key(slot_id, "ed25519-signing", HsmKeyAlgorithm::Ed25519, tick)
}

pub fn default_slot() -> HsmSlot {
    let mut slot = HsmSlot::new(0, "Primary Slot", "SoftHSM2");
    slot.insert_token();
    slot
}

pub fn strict_hsm_policy() -> HsmPolicy {
    HsmPolicy::new()
        .block_algorithm(HsmKeyAlgorithm::Rsa2048)
        .min_key_bits(256)
}
