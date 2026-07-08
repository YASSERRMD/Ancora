use crate::key::CryptoKey;
use crate::store::KeyStore;

pub struct ExpiryChecker;

impl ExpiryChecker {
    pub fn expired_keys<'a>(
        store: &'a KeyStore,
        tenant_id: &str,
        current_tick: u64,
    ) -> Vec<&'a CryptoKey> {
        store
            .list_tenant_active(tenant_id)
            .into_iter()
            .filter(|k| k.is_expired(current_tick))
            .collect()
    }

    pub fn expiring_soon<'a>(
        store: &'a KeyStore,
        tenant_id: &str,
        current_tick: u64,
        warning_ticks: u64,
    ) -> Vec<&'a CryptoKey> {
        store
            .list_tenant_active(tenant_id)
            .into_iter()
            .filter(|k| {
                if let Some(exp) = k.expires_tick {
                    exp > current_tick && exp <= current_tick + warning_ticks
                } else {
                    false
                }
            })
            .collect()
    }
}
