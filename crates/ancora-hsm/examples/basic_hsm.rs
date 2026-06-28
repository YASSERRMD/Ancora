use ancora_hsm::audit::{HsmAuditEntry, HsmAuditLog, HsmOperation};
use ancora_hsm::mock::SoftHsm;
use ancora_hsm::presets::{aes256_key, default_slot, ed25519_signing_key, strict_hsm_policy};
use ancora_hsm::report::HsmReport;
use ancora_hsm::session::SessionManager;
use ancora_hsm::slot::SlotManager;

fn main() {
    let mut hsm = SoftHsm::new();
    let policy = strict_hsm_policy();

    let aes_handle = aes256_key(&mut hsm, 0, 1);
    let sign_handle = ed25519_signing_key(&mut hsm, 0, 2);

    let aes_key = hsm.get_key(aes_handle).unwrap();
    let sign_key = hsm.get_key(sign_handle).unwrap();
    println!("AES key allowed: {}", policy.is_allowed(aes_key));
    println!("Signing key allowed: {}", policy.is_allowed(sign_key));

    let message = b"hello ancora hsm";
    let sig = hsm.sign(sign_handle, message).unwrap();
    println!("Signature length: {} bytes", sig.len());

    let encrypted = hsm.encrypt(aes_handle, message).unwrap();
    let decrypted = hsm.decrypt(aes_handle, &encrypted).unwrap();
    assert_eq!(&decrypted, message);
    println!("Encrypt/decrypt roundtrip: ok");

    let mut slots = SlotManager::new();
    slots.add_slot(default_slot());
    let sessions = SessionManager::new();

    let mut audit = HsmAuditLog::new();
    audit.record(HsmAuditEntry::new(1, 0, HsmOperation::GenerateKey, true, "aes256"));
    audit.record(HsmAuditEntry::new(2, 0, HsmOperation::GenerateKey, true, "ed25519"));
    audit.record(HsmAuditEntry::new(3, 0, HsmOperation::Sign, true, "signed"));

    let report = HsmReport::generate(&hsm, &slots, &sessions, &audit, 100);
    println!("Total slots: {}", report.total_slots);
    println!("Slots with token: {}", report.slots_with_token);
    println!("Total keys: {}", report.total_keys);
    println!("Total HSM operations: {}", report.total_operations);
    println!("Audit failures: {}", report.audit_failures);
}
