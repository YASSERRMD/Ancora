use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaType {
    UsbDrive,
    CdRom,
    DvdRom,
    NetworkShare,
    Bluetooth,
    PrintedDocument,
    OpticalFibre,
    HardDrive,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MediaType::UsbDrive => "USB_DRIVE",
            MediaType::CdRom => "CD_ROM",
            MediaType::DvdRom => "DVD_ROM",
            MediaType::NetworkShare => "NETWORK_SHARE",
            MediaType::Bluetooth => "BLUETOOTH",
            MediaType::PrintedDocument => "PRINTED_DOCUMENT",
            MediaType::OpticalFibre => "OPTICAL_FIBRE",
            MediaType::HardDrive => "HARD_DRIVE",
        };
        f.write_str(s)
    }
}

pub struct MediaControl {
    pub tenant_id: String,
    allowed: HashSet<MediaType>,
    blocked: HashSet<MediaType>,
}

impl MediaControl {
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            allowed: HashSet::new(),
            blocked: HashSet::new(),
        }
    }

    pub fn allow(mut self, media: MediaType) -> Self {
        self.blocked.remove(&media);
        self.allowed.insert(media);
        self
    }

    pub fn block(mut self, media: MediaType) -> Self {
        self.allowed.remove(&media);
        self.blocked.insert(media);
        self
    }

    pub fn is_allowed(&self, media: &MediaType) -> bool {
        if self.blocked.contains(media) {
            return false;
        }
        self.allowed.contains(media)
    }

    pub fn is_blocked(&self, media: &MediaType) -> bool {
        self.blocked.contains(media)
    }

    pub fn allowed_count(&self) -> usize {
        self.allowed.len()
    }
    pub fn blocked_count(&self) -> usize {
        self.blocked.len()
    }
}
