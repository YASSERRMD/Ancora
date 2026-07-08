use crate::audit::{KeyAuditEntry, KeyAuditLog, KeyOperation};
use crate::store::{KeyStore, KeyStoreError};

pub fn deactivate_key(
    store: &mut KeyStore,
    tenant_id: &str,
    key_id: &str,
    subject: &str,
    tick: u64,
    audit: &mut KeyAuditLog,
) -> Result<(), KeyStoreError> {
    let key = store.get_latest_mut(tenant_id, key_id)?;
    let version = key.version;
    key.deactivate();
    audit.record(KeyAuditEntry::new(
        tick,
        tenant_id,
        key_id,
        version,
        KeyOperation::Deactivate,
        subject,
        true,
    ));
    Ok(())
}

pub fn compromise_key(
    store: &mut KeyStore,
    tenant_id: &str,
    key_id: &str,
    subject: &str,
    tick: u64,
    audit: &mut KeyAuditLog,
) -> Result<(), KeyStoreError> {
    let key = store.get_latest_mut(tenant_id, key_id)?;
    let version = key.version;
    key.mark_compromised();
    audit.record(KeyAuditEntry::new(
        tick,
        tenant_id,
        key_id,
        version,
        KeyOperation::Compromise,
        subject,
        true,
    ));
    Ok(())
}

pub fn destroy_key(
    store: &mut KeyStore,
    tenant_id: &str,
    key_id: &str,
    subject: &str,
    tick: u64,
    audit: &mut KeyAuditLog,
) -> Result<(), KeyStoreError> {
    let key = store.get_latest_mut(tenant_id, key_id)?;
    let version = key.version;
    key.destroy();
    audit.record(KeyAuditEntry::new(
        tick,
        tenant_id,
        key_id,
        version,
        KeyOperation::Destroy,
        subject,
        true,
    ));
    Ok(())
}

pub fn schedule_key_deletion(
    store: &mut KeyStore,
    tenant_id: &str,
    key_id: &str,
    subject: &str,
    tick: u64,
    audit: &mut KeyAuditLog,
) -> Result<(), KeyStoreError> {
    let key = store.get_latest_mut(tenant_id, key_id)?;
    let version = key.version;
    key.schedule_deletion();
    audit.record(KeyAuditEntry::new(
        tick,
        tenant_id,
        key_id,
        version,
        KeyOperation::Deactivate,
        subject,
        true,
    ));
    Ok(())
}
