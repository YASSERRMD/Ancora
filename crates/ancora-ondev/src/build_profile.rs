//! Minimal runtime build profile for on-device deployment.
//!
//! Controls compile-time optimizations and footprint targets for
//! ARM and mobile platforms.

use serde::{Deserialize, Serialize};

/// Build optimization level for on-device targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptLevel {
    /// Optimise for minimum binary size (`opt-level = "z"`).
    Size,
    /// Balance size and speed (`opt-level = "s"`).
    SizeSpeed,
    /// Maximum speed (`opt-level = 3`).
    Speed,
}

impl Default for OptLevel {
    fn default() -> Self {
        Self::Size
    }
}

impl std::fmt::Display for OptLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Size => write!(f, "z"),
            Self::SizeSpeed => write!(f, "s"),
            Self::Speed => write!(f, "3"),
        }
    }
}

/// Panic strategy for on-device builds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanicStrategy {
    /// Unwind the stack (default, larger binary).
    Unwind,
    /// Abort immediately (smaller binary, preferred for embedded).
    Abort,
}

impl Default for PanicStrategy {
    fn default() -> Self {
        Self::Abort
    }
}

/// LTO (Link-Time Optimisation) mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LtoMode {
    /// No LTO.
    Off,
    /// Thin LTO (fast, moderate size reduction).
    Thin,
    /// Full LTO (slow, maximum size reduction).
    Full,
}

impl Default for LtoMode {
    fn default() -> Self {
        Self::Full
    }
}

/// Complete build profile configuration for on-device targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfile {
    /// Name of this profile.
    pub name: String,
    /// Optimisation level.
    pub opt_level: OptLevel,
    /// Panic strategy.
    pub panic: PanicStrategy,
    /// LTO mode.
    pub lto: LtoMode,
    /// Strip debug symbols.
    pub strip: bool,
    /// Codegen units (1 = best optimisation, more = faster compile).
    pub codegen_units: u32,
    /// Target binary size budget in bytes (0 = no limit).
    pub size_budget_bytes: usize,
}

impl Default for BuildProfile {
    fn default() -> Self {
        Self {
            name: "ondev".to_string(),
            opt_level: OptLevel::Size,
            panic: PanicStrategy::Abort,
            lto: LtoMode::Full,
            strip: true,
            codegen_units: 1,
            size_budget_bytes: 5 * 1024 * 1024, // 5 MiB
        }
    }
}

impl BuildProfile {
    /// Create a minimal profile targeting the smallest possible binary.
    pub fn minimal() -> Self {
        Self::default()
    }

    /// Create a profile balancing size and runtime performance.
    pub fn balanced() -> Self {
        Self {
            name: "ondev-balanced".to_string(),
            opt_level: OptLevel::SizeSpeed,
            lto: LtoMode::Thin,
            codegen_units: 4,
            size_budget_bytes: 10 * 1024 * 1024,
            ..Self::default()
        }
    }

    /// Check whether the profile meets its size budget.
    ///
    /// Returns `true` when `actual_bytes` is within the budget (or there is no
    /// budget set).
    pub fn within_budget(&self, actual_bytes: usize) -> bool {
        self.size_budget_bytes == 0 || actual_bytes <= self.size_budget_bytes
    }

    /// Render a `Cargo.toml` `[profile.*]` snippet for this configuration.
    pub fn cargo_toml_snippet(&self) -> String {
        format!(
            "[profile.{}]\nopt-level = \"{}\"\npanic = \"{}\"\nlto = {}\nstrip = {}\ncodegen-units = {}\n",
            self.name,
            self.opt_level,
            match self.panic { PanicStrategy::Abort => "abort", PanicStrategy::Unwind => "unwind" },
            match self.lto { LtoMode::Off => "false", LtoMode::Thin => "\"thin\"", LtoMode::Full => "true" },
            self.strip,
            self.codegen_units,
        )
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn default_profile_is_minimal() {
        let p = BuildProfile::minimal();
        assert_eq!(p.opt_level, OptLevel::Size);
        assert_eq!(p.panic, PanicStrategy::Abort);
        assert_eq!(p.lto, LtoMode::Full);
        assert!(p.strip);
        assert_eq!(p.codegen_units, 1);
    }

    #[test]
    fn within_budget_returns_correct_result() {
        let p = BuildProfile::minimal();
        assert!(p.within_budget(1024));
        assert!(!p.within_budget(6 * 1024 * 1024));
    }

    #[test]
    fn cargo_snippet_contains_profile_name() {
        let p = BuildProfile::minimal();
        let snip = p.cargo_toml_snippet();
        assert!(snip.contains("[profile.ondev]"));
        assert!(snip.contains("opt-level = \"z\""));
    }

    #[test]
    fn minimal_profile_size_within_5mib_budget() {
        let p = BuildProfile::minimal();
        // The source of this crate is <1 MiB; well within the 5 MiB budget.
        assert!(p.within_budget(512 * 1024));
    }

    #[test]
    fn profile_no_budget_always_passes() {
        let mut p = BuildProfile::minimal();
        p.size_budget_bytes = 0;
        assert!(p.within_budget(usize::MAX));
    }

    #[test]
    fn balanced_profile_lto_is_thin() {
        let p = BuildProfile::balanced();
        assert_eq!(p.lto, LtoMode::Thin);
    }
}
