use crate::media::MediaType;

#[test]
fn display_usb_drive() {
    assert_eq!(format!("{}", MediaType::UsbDrive), "USB_DRIVE");
}

#[test]
fn display_cd_rom() {
    assert_eq!(format!("{}", MediaType::CdRom), "CD_ROM");
}

#[test]
fn display_dvd_rom() {
    assert_eq!(format!("{}", MediaType::DvdRom), "DVD_ROM");
}

#[test]
fn display_network_share() {
    assert_eq!(format!("{}", MediaType::NetworkShare), "NETWORK_SHARE");
}

#[test]
fn display_bluetooth() {
    assert_eq!(format!("{}", MediaType::Bluetooth), "BLUETOOTH");
}

#[test]
fn display_printed_document() {
    assert_eq!(format!("{}", MediaType::PrintedDocument), "PRINTED_DOCUMENT");
}

#[test]
fn display_optical_fibre() {
    assert_eq!(format!("{}", MediaType::OpticalFibre), "OPTICAL_FIBRE");
}

#[test]
fn display_hard_drive() {
    assert_eq!(format!("{}", MediaType::HardDrive), "HARD_DRIVE");
}
