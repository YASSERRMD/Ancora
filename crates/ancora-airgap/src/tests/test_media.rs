use crate::media::{MediaControl, MediaType};

#[test]
fn media_control_allow() {
    let mc = MediaControl::new("t1").allow(MediaType::UsbDrive);
    assert!(mc.is_allowed(&MediaType::UsbDrive));
    assert!(!mc.is_allowed(&MediaType::Bluetooth));
}

#[test]
fn media_control_block() {
    let mc = MediaControl::new("t1").block(MediaType::Bluetooth);
    assert!(mc.is_blocked(&MediaType::Bluetooth));
    assert!(!mc.is_allowed(&MediaType::Bluetooth));
}

#[test]
fn media_control_allow_and_block() {
    let mc = MediaControl::new("t1")
        .allow(MediaType::UsbDrive)
        .block(MediaType::NetworkShare);
    assert_eq!(mc.allowed_count(), 1);
    assert_eq!(mc.blocked_count(), 1);
}
