use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Untrusted,
    Partial,
    Trusted,
    FullyTrusted,
}

impl fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TrustLevel::Untrusted => "UNTRUSTED",
            TrustLevel::Partial => "PARTIAL",
            TrustLevel::Trusted => "TRUSTED",
            TrustLevel::FullyTrusted => "FULLY_TRUSTED",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct DevicePosture {
    pub device_id: String,
    pub tenant_id: String,
    pub trust_level: TrustLevel,
    pub os_up_to_date: bool,
    pub antivirus_active: bool,
    pub disk_encrypted: bool,
    pub last_checked_tick: u64,
}

impl DevicePosture {
    pub fn new(device_id: impl Into<String>, tenant_id: impl Into<String>, tick: u64) -> Self {
        Self {
            device_id: device_id.into(),
            tenant_id: tenant_id.into(),
            trust_level: TrustLevel::Untrusted,
            os_up_to_date: false,
            antivirus_active: false,
            disk_encrypted: false,
            last_checked_tick: tick,
        }
    }

    pub fn compute_trust(&mut self) {
        let score =
            self.os_up_to_date as u8 + self.antivirus_active as u8 + self.disk_encrypted as u8;
        self.trust_level = match score {
            3 => TrustLevel::FullyTrusted,
            2 => TrustLevel::Trusted,
            1 => TrustLevel::Partial,
            _ => TrustLevel::Untrusted,
        };
    }

    pub fn is_trusted(&self) -> bool {
        self.trust_level >= TrustLevel::Trusted
    }
}

pub struct DeviceStore {
    devices: std::collections::HashMap<String, DevicePosture>,
}

impl DeviceStore {
    pub fn new() -> Self {
        Self {
            devices: std::collections::HashMap::new(),
        }
    }
    pub fn upsert(&mut self, posture: DevicePosture) {
        self.devices.insert(posture.device_id.clone(), posture);
    }
    pub fn get(&self, device_id: &str) -> Option<&DevicePosture> {
        self.devices.get(device_id)
    }
    pub fn get_mut(&mut self, device_id: &str) -> Option<&mut DevicePosture> {
        self.devices.get_mut(device_id)
    }
    pub fn trusted(&self) -> Vec<&DevicePosture> {
        self.devices.values().filter(|d| d.is_trusted()).collect()
    }
    pub fn for_tenant(&self, tenant_id: &str) -> Vec<&DevicePosture> {
        self.devices
            .values()
            .filter(|d| d.tenant_id == tenant_id)
            .collect()
    }
    pub fn count(&self) -> usize {
        self.devices.len()
    }
}
