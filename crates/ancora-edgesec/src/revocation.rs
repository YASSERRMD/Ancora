use std::collections::{HashMap, HashSet};
use std::fmt;

/// Reason for revoking a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevocationReason {
    KeyCompromised,
    TamperDetected,
    PolicyViolation,
    Decommissioned,
    UnknownReason,
}

impl fmt::Display for RevocationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RevocationReason::KeyCompromised => "KEY_COMPROMISED",
            RevocationReason::TamperDetected => "TAMPER_DETECTED",
            RevocationReason::PolicyViolation => "POLICY_VIOLATION",
            RevocationReason::Decommissioned => "DECOMMISSIONED",
            RevocationReason::UnknownReason => "UNKNOWN",
        };
        f.write_str(s)
    }
}

/// A revocation record for a device.
#[derive(Debug, Clone)]
pub struct RevocationRecord {
    pub device_id: String,
    pub reason: RevocationReason,
    pub detail: String,
    pub tick: u64,
}

impl RevocationRecord {
    pub fn new(
        device_id: impl Into<String>,
        reason: RevocationReason,
        detail: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            device_id: device_id.into(),
            reason,
            detail: detail.into(),
            tick,
        }
    }
}

/// Device revocation list (CRL equivalent for edge devices).
pub struct DeviceRevocationList {
    revoked: HashSet<String>,
    records: HashMap<String, RevocationRecord>,
}

impl Default for DeviceRevocationList {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceRevocationList {
    pub fn new() -> Self {
        Self {
            revoked: HashSet::new(),
            records: HashMap::new(),
        }
    }

    /// Revoke a device.
    pub fn revoke(
        &mut self,
        device_id: impl Into<String>,
        reason: RevocationReason,
        detail: impl Into<String>,
        tick: u64,
    ) {
        let id: String = device_id.into();
        let record = RevocationRecord::new(id.clone(), reason, detail, tick);
        self.revoked.insert(id.clone());
        self.records.insert(id, record);
    }

    /// Returns true if the device is revoked.
    pub fn is_revoked(&self, device_id: &str) -> bool {
        self.revoked.contains(device_id)
    }

    /// Get the revocation record for a device.
    pub fn get_record(&self, device_id: &str) -> Option<&RevocationRecord> {
        self.records.get(device_id)
    }

    /// Total number of revoked devices.
    pub fn revoked_count(&self) -> usize {
        self.revoked.len()
    }

    /// Iterate over all revocation records.
    pub fn all_records(&self) -> impl Iterator<Item = &RevocationRecord> {
        self.records.values()
    }

    /// Remove a device from the revocation list (re-enroll scenario).
    pub fn unrevoke(&mut self, device_id: &str) -> bool {
        let removed = self.revoked.remove(device_id);
        self.records.remove(device_id);
        removed
    }
}

impl Default for DeviceRevocationList {
    fn default() -> Self {
        Self::new()
    }
}
