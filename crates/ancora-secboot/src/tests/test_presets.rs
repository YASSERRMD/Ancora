use crate::{permissive_boot_policy, strict_boot_policy, kernel_only_policy};
#[test]
fn strict_policy_requires_firmware_bootloader_kernel() {
    let p = strict_boot_policy("t1");
    assert!(p.require_kinds.contains("FIRMWARE"));
    assert!(p.require_kinds.contains("BOOTLOADER"));
    assert!(p.require_kinds.contains("KERNEL"));
}
#[test]
fn permissive_policy_allows_unknown_digests() {
    let p = permissive_boot_policy("t1");
    assert!(!p.deny_unknown);
}
#[test]
fn kernel_only_policy_requires_only_kernel() {
    let p = kernel_only_policy("t1");
    assert!(p.require_kinds.contains("KERNEL"));
    assert_eq!(p.require_kinds.len(), 1);
}
