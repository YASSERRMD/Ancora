use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementKind {
    Firmware,
    Bootloader,
    Kernel,
    InitRamdisk,
    ConfigFile,
    Application,
}

impl fmt::Display for MeasurementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MeasurementKind::Firmware => "FIRMWARE",
            MeasurementKind::Bootloader => "BOOTLOADER",
            MeasurementKind::Kernel => "KERNEL",
            MeasurementKind::InitRamdisk => "INIT_RAMDISK",
            MeasurementKind::ConfigFile => "CONFIG_FILE",
            MeasurementKind::Application => "APPLICATION",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub id: String,
    pub kind: MeasurementKind,
    pub name: String,
    pub digest: String,
    pub tick: u64,
    pub metadata: HashMap<String, String>,
}

impl Measurement {
    pub fn new(
        id: impl Into<String>,
        kind: MeasurementKind,
        name: impl Into<String>,
        digest: impl Into<String>,
        tick: u64,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            name: name.into(),
            digest: digest.into(),
            tick,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn matches_digest(&self, expected: &str) -> bool {
        self.digest == expected
    }
}
