use crate::media::{MediaControl, MediaType};

#[test]
fn block_overrides_allow() {
    let mc = MediaControl::new("t1")
        .allow(MediaType::UsbDrive)
        .block(MediaType::UsbDrive);
    assert!(!mc.is_allowed(&MediaType::UsbDrive));
    assert!(mc.is_blocked(&MediaType::UsbDrive));
}

#[test]
fn unknown_media_not_allowed() {
    let mc = MediaControl::new("t1").allow(MediaType::CdRom);
    assert!(!mc.is_allowed(&MediaType::Bluetooth));
}

#[test]
fn counts_accurate() {
    let mc = MediaControl::new("t1")
        .allow(MediaType::CdRom)
        .allow(MediaType::DvdRom)
        .block(MediaType::Bluetooth);
    assert_eq!(mc.allowed_count(), 2);
    assert_eq!(mc.blocked_count(), 1);
}
