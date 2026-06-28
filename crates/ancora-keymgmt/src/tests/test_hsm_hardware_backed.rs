use crate::{HsmBackend, HsmConfig};
#[test]
fn software_backend_is_not_hardware_backed() {
    let config = HsmConfig::software();
    assert!(!config.is_hardware_backed());
}
#[test]
fn cloud_kms_backend_is_hardware_backed() {
    let config = HsmConfig::cloud_kms(1);
    assert!(config.is_hardware_backed());
    assert_eq!(config.backend, HsmBackend::CloudKms);
}
