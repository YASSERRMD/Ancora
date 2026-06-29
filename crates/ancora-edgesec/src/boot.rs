use std::fmt;

/// Status of a secure boot attestation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootStatus {
    Verified,
    Tampered,
    Unknown,
}

impl fmt::Display for BootStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BootStatus::Verified => "VERIFIED",
            BootStatus::Tampered => "TAMPERED",
            BootStatus::Unknown => "UNKNOWN",
        };
        f.write_str(s)
    }
}

/// A secure boot measurement represents the hash of a boot stage component.
#[derive(Debug, Clone)]
pub struct BootMeasurement {
    pub component: String,
    /// Expected hash (simulated as 32 bytes).
    pub expected_hash: Vec<u8>,
    /// Actual hash measured at boot time.
    pub measured_hash: Vec<u8>,
}

impl BootMeasurement {
    pub fn new(
        component: impl Into<String>,
        expected_hash: Vec<u8>,
        measured_hash: Vec<u8>,
    ) -> Self {
        Self {
            component: component.into(),
            expected_hash,
            measured_hash,
        }
    }

    /// Returns true if the measurement matches expectations.
    pub fn is_valid(&self) -> bool {
        self.expected_hash == self.measured_hash
    }
}

/// A secure boot attestation record for an edge device.
#[derive(Debug, Clone)]
pub struct SecureBootAttestation {
    pub device_id: String,
    pub measurements: Vec<BootMeasurement>,
    pub status: BootStatus,
    pub tick: u64,
}

impl SecureBootAttestation {
    /// Create a new attestation by running the boot hook over the provided measurements.
    pub fn attest(device_id: impl Into<String>, measurements: Vec<BootMeasurement>, tick: u64) -> Self {
        let all_valid = measurements.iter().all(|m| m.is_valid());
        let status = if measurements.is_empty() {
            BootStatus::Unknown
        } else if all_valid {
            BootStatus::Verified
        } else {
            BootStatus::Tampered
        };
        Self {
            device_id: device_id.into(),
            measurements,
            status,
            tick,
        }
    }

    /// Returns true if boot was verified.
    pub fn is_verified(&self) -> bool {
        self.status == BootStatus::Verified
    }

    /// Returns a list of failed component names.
    pub fn failed_components(&self) -> Vec<&str> {
        self.measurements
            .iter()
            .filter(|m| !m.is_valid())
            .map(|m| m.component.as_str())
            .collect()
    }
}

/// A hook that can be registered to run a pre-boot integrity check.
pub struct SecureBootHook {
    pub hook_name: String,
    hook_fn: Box<dyn Fn(&[BootMeasurement]) -> bool + Send + Sync>,
}

impl SecureBootHook {
    pub fn new(
        hook_name: impl Into<String>,
        hook_fn: impl Fn(&[BootMeasurement]) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            hook_name: hook_name.into(),
            hook_fn: Box::new(hook_fn),
        }
    }

    /// Run the hook and return true if boot is allowed.
    pub fn run(&self, measurements: &[BootMeasurement]) -> bool {
        (self.hook_fn)(measurements)
    }
}
