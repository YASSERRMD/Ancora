use crate::boot::BootStatus;
use crate::tamper::TamperEvent;

/// Summary status for an attestation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Clean,
    Warning,
    Compromised,
}

impl std::fmt::Display for ReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportStatus::Clean => f.write_str("CLEAN"),
            ReportStatus::Warning => f.write_str("WARNING"),
            ReportStatus::Compromised => f.write_str("COMPROMISED"),
        }
    }
}

/// A remote attestation report for an edge device.
#[derive(Debug, Clone)]
pub struct AttestationReport {
    pub device_id: String,
    pub tick: u64,
    pub boot_status: BootStatus,
    pub model_valid: bool,
    pub config_valid: bool,
    pub tamper_events: Vec<TamperEvent>,
    pub status: ReportStatus,
    pub nonce: u64,
}

impl AttestationReport {
    /// Build a remote attestation report.
    pub fn generate(
        device_id: impl Into<String>,
        tick: u64,
        boot_status: BootStatus,
        model_valid: bool,
        config_valid: bool,
        tamper_events: Vec<TamperEvent>,
        nonce: u64,
    ) -> Self {
        let status = if !tamper_events.is_empty() || boot_status == BootStatus::Tampered {
            ReportStatus::Compromised
        } else if !model_valid || !config_valid {
            ReportStatus::Warning
        } else {
            ReportStatus::Clean
        };
        Self {
            device_id: device_id.into(),
            tick,
            boot_status,
            model_valid,
            config_valid,
            tamper_events,
            status,
            nonce,
        }
    }

    /// Returns true if the device passed all checks.
    pub fn is_clean(&self) -> bool {
        self.status == ReportStatus::Clean
    }

    /// Number of tamper events captured.
    pub fn tamper_count(&self) -> usize {
        self.tamper_events.len()
    }

    /// Serialize the report to a simple key=value string (no external deps).
    pub fn to_text(&self) -> String {
        format!(
            "device_id={}\ntick={}\nboot_status={}\nmodel_valid={}\nconfig_valid={}\ntamper_events={}\nstatus={}\nnonce={}\n",
            self.device_id,
            self.tick,
            self.boot_status,
            self.model_valid,
            self.config_valid,
            self.tamper_events.len(),
            self.status,
            self.nonce,
        )
    }
}
