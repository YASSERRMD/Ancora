//! Target triple definitions for ARM and mobile platforms.
//!
//! Covers ARM64 Linux, ARM32 Linux (hard-float), Android JNI (arm64-v8a),
//! and iOS C-ABI (aarch64-apple-ios).

use serde::{Deserialize, Serialize};

/// The set of supported on-device target triples.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetTriple {
    /// 64-bit ARM Linux (musl libc for static linking).
    Arm64Linux,
    /// 32-bit ARM Linux hard-float (Cortex-A class).
    Arm32Linux,
    /// Android arm64-v8a with JNI bridge.
    AndroidArm64,
    /// iOS aarch64 C-ABI (simulator and device).
    IosArm64,
    /// Generic host — used for local testing.
    Host,
}

impl TargetTriple {
    /// Return the Rust target triple string.
    pub fn triple(&self) -> &'static str {
        match self {
            Self::Arm64Linux => "aarch64-unknown-linux-musl",
            Self::Arm32Linux => "armv7-unknown-linux-musleabihf",
            Self::AndroidArm64 => "aarch64-linux-android",
            Self::IosArm64 => "aarch64-apple-ios",
            Self::Host => std::env::consts::ARCH,
        }
    }

    /// Return the linker that should be used for this target.
    pub fn linker(&self) -> Option<&'static str> {
        match self {
            Self::Arm64Linux => Some("aarch64-linux-musl-gcc"),
            Self::Arm32Linux => Some("arm-linux-musleabihf-gcc"),
            Self::AndroidArm64 => Some("aarch64-linux-android-clang"),
            Self::IosArm64 => None, // handled by cargo-bundle / xcodebuild
            Self::Host => None,
        }
    }

    /// Whether this target supports the JNI bridge.
    pub fn has_jni(&self) -> bool {
        matches!(self, Self::AndroidArm64)
    }

    /// Whether this target uses the iOS C-ABI.
    pub fn has_ios_cabi(&self) -> bool {
        matches!(self, Self::IosArm64)
    }

    /// Whether this target is 64-bit ARM.
    pub fn is_arm64(&self) -> bool {
        matches!(self, Self::Arm64Linux | Self::AndroidArm64 | Self::IosArm64)
    }

    /// Whether this target is 32-bit ARM.
    pub fn is_arm32(&self) -> bool {
        matches!(self, Self::Arm32Linux)
    }
}

impl std::fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.triple())
    }
}

/// Target-specific build metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMeta {
    /// The target triple.
    pub target: TargetTriple,
    /// CPU features to enable (e.g., `+neon`, `+fp16`).
    pub cpu_features: Vec<String>,
    /// Whether to enable position-independent code.
    pub pic: bool,
    /// Minimum OS / API level (Android SDK level or iOS version string).
    pub min_sdk: Option<String>,
}

impl TargetMeta {
    /// Create default metadata for a given target.
    pub fn for_target(target: TargetTriple) -> Self {
        let cpu_features = match &target {
            TargetTriple::Arm64Linux | TargetTriple::AndroidArm64 | TargetTriple::IosArm64 => {
                vec!["+neon".to_string(), "+fp-armv8".to_string()]
            }
            TargetTriple::Arm32Linux => vec!["+neon".to_string(), "+vfpv3".to_string()],
            TargetTriple::Host => vec![],
        };
        let min_sdk = match &target {
            TargetTriple::AndroidArm64 => Some("21".to_string()),
            TargetTriple::IosArm64 => Some("14.0".to_string()),
            _ => None,
        };
        Self { target, cpu_features, pic: true, min_sdk }
    }
}

/// ARM32-specific build hints (Thumb-2 ISA, VFPv3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arm32Config {
    /// Enable Thumb-2 instruction set.
    pub thumb2: bool,
    /// Enable VFPv3-D16 floating-point.
    pub vfpv3: bool,
    /// Minimum ARM architecture version.
    pub arch_version: u8,
}

impl Default for Arm32Config {
    fn default() -> Self {
        Self { thumb2: true, vfpv3: true, arch_version: 7 }
    }
}

impl Arm32Config {
    /// Return the `-C target-feature` string.
    pub fn target_feature_string(&self) -> String {
        let mut features = Vec::new();
        if self.thumb2 {
            features.push("+thumb2");
        }
        if self.vfpv3 {
            features.push("+vfpv3");
        }
        features.join(",")
    }
}

/// JNI bridge configuration for Android targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JniBridge {
    /// Java package name (e.g., `com.ancora.agent`).
    pub package: String,
    /// Native class name.
    pub class: String,
}

impl JniBridge {
    /// Create a default JNI bridge configuration.
    pub fn default_config() -> Self {
        Self {
            package: "com.ancora.agent".to_string(),
            class: "AgentRuntime".to_string(),
        }
    }

    /// Return the JNI function name prefix.
    pub fn jni_prefix(&self) -> String {
        let pkg = self.package.replace('.', "_");
        format!("Java_{}_{}", pkg, self.class)
    }
}

/// Android-specific build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidConfig {
    /// NDK version string (e.g., `"r25c"`).
    pub ndk_version: String,
    /// Android API level.
    pub api_level: u32,
    /// ABI filter list.
    pub abis: Vec<String>,
}

impl Default for AndroidConfig {
    fn default() -> Self {
        Self {
            ndk_version: "r25c".to_string(),
            api_level: 21,
            abis: vec!["arm64-v8a".to_string()],
        }
    }
}

impl AndroidConfig {
    /// Return the primary ABI.
    pub fn primary_abi(&self) -> &str {
        self.abis.first().map(|s| s.as_str()).unwrap_or("arm64-v8a")
    }
}

/// iOS C-ABI export descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosCabi {
    /// Module name exposed to Swift / ObjC.
    pub module_name: String,
    /// Header path for the generated umbrella header.
    pub header_path: String,
    /// Minimum iOS deployment target.
    pub min_ios_version: String,
    /// Whether to build a universal (simulator + device) XCFramework.
    pub xcframework: bool,
}

impl IosCabi {
    /// Create a default iOS C-ABI configuration.
    pub fn default_config() -> Self {
        Self {
            module_name: "AncoraOndev".to_string(),
            header_path: "include/ancora_ondev.h".to_string(),
            min_ios_version: "14.0".to_string(),
            xcframework: false,
        }
    }

    /// Return the Swift module map content.
    pub fn module_map(&self) -> String {
        format!(
            "module {} {{\n  umbrella header \"{}\"\n  export *\n}}\n",
            self.module_name, self.header_path
        )
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn arm64_linux_triple() {
        let t = TargetTriple::Arm64Linux;
        assert_eq!(t.triple(), "aarch64-unknown-linux-musl");
        assert!(t.is_arm64());
        assert!(!t.is_arm32());
    }

    #[test]
    fn arm32_linux_triple() {
        let t = TargetTriple::Arm32Linux;
        assert_eq!(t.triple(), "armv7-unknown-linux-musleabihf");
        assert!(t.is_arm32());
        assert!(!t.is_arm64());
    }

    #[test]
    fn android_has_jni() {
        let t = TargetTriple::AndroidArm64;
        assert!(t.has_jni());
        assert!(!t.has_ios_cabi());
    }

    #[test]
    fn ios_has_cabi() {
        let t = TargetTriple::IosArm64;
        assert!(t.has_ios_cabi());
        assert!(!t.has_jni());
    }

    #[test]
    fn jni_prefix_correct() {
        let b = JniBridge::default_config();
        assert_eq!(b.jni_prefix(), "Java_com_ancora_agent_AgentRuntime");
    }

    #[test]
    fn target_meta_android_sdk_level() {
        let m = TargetMeta::for_target(TargetTriple::AndroidArm64);
        assert_eq!(m.min_sdk, Some("21".to_string()));
    }
}
