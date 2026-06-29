//! Android JNI target loader and integration helpers.
//!
//! Provides types and utilities for loading the on-device runtime
//! inside an Android application via the JNI bridge.

use crate::targets::{AndroidConfig, JniBridge, TargetTriple};
use serde::{Deserialize, Serialize};

/// State returned when the Android JNI runtime loads successfully.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidLoadResult {
    /// Target triple that was resolved at runtime.
    pub triple: String,
    /// Android API level detected on the device.
    pub api_level: u32,
    /// Primary ABI of the running process.
    pub primary_abi: String,
    /// Whether the JNI bridge is initialised.
    pub jni_ready: bool,
}

/// Simulate loading the on-device runtime for Android.
///
/// In production this would be called from `JNI_OnLoad` to register
/// native methods.  Here we validate configuration and return a
/// load descriptor.
pub fn android_load(config: &AndroidConfig, bridge: &JniBridge) -> AndroidLoadResult {
    AndroidLoadResult {
        triple: TargetTriple::AndroidArm64.triple().to_string(),
        api_level: config.api_level,
        primary_abi: config.primary_abi().to_string(),
        jni_ready: !bridge.jni_prefix().is_empty(),
    }
}

/// Check whether a given API level is supported.
pub fn api_level_supported(level: u32) -> bool {
    level >= 21 // Android 5.0 Lollipop minimum
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn android_load_returns_ready() {
        let cfg = AndroidConfig::default();
        let bridge = JniBridge::default_config();
        let result = android_load(&cfg, &bridge);
        assert!(result.jni_ready);
        assert_eq!(result.api_level, 21);
    }

    #[test]
    fn api_level_21_is_supported() {
        assert!(api_level_supported(21));
        assert!(api_level_supported(33));
        assert!(!api_level_supported(19));
    }
}
