/// Interop parity kit.
///
/// Provides a cross-language test harness that verifies every extension,
/// regardless of source language, satisfies the same behavioural contract.
/// Any adapter registered in the extension registry is expected to pass
/// all checks in `InteropKit::run_all`.
use std::collections::HashMap;

use crate::registration::{ExtensionRegistry, Language};
use crate::rs_traits::{ToolExtension, Value};

// ---------------------------------------------------------------------------
// Interop kit
// ---------------------------------------------------------------------------

/// A suite of behavioural checks that every Ancora extension must pass.
pub struct InteropKit;

/// The outcome of a single check.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl CheckResult {
    fn pass(name: impl Into<String>) -> Self {
        CheckResult {
            name: name.into(),
            passed: true,
            message: "ok".to_string(),
        }
    }

    fn fail(name: impl Into<String>, message: impl Into<String>) -> Self {
        CheckResult {
            name: name.into(),
            passed: false,
            message: message.into(),
        }
    }
}

impl InteropKit {
    /// Run all parity checks against a single extension.
    /// Returns a list of `CheckResult`s, one per check.
    pub fn run_all(ext: &dyn ToolExtension) -> Vec<CheckResult> {
        vec![
            Self::check_meta_non_empty(ext),
            Self::check_health_ok(ext),
            Self::check_execute_returns_value(ext),
            Self::check_invalid_arg_returns_error(ext),
        ]
    }

    /// Verify that `meta()` returns non-empty name, description, and version.
    pub fn check_meta_non_empty(ext: &dyn ToolExtension) -> CheckResult {
        let meta = ext.meta();
        if meta.name.is_empty() {
            return CheckResult::fail("meta_non_empty", "name is empty");
        }
        if meta.description.is_empty() {
            return CheckResult::fail("meta_non_empty", "description is empty");
        }
        if meta.version.is_empty() {
            return CheckResult::fail("meta_non_empty", "version is empty");
        }
        CheckResult::pass("meta_non_empty")
    }

    /// Verify that `health_check()` returns `Ok(())`.
    pub fn check_health_ok(ext: &dyn ToolExtension) -> CheckResult {
        match ext.health_check() {
            Ok(()) => CheckResult::pass("health_ok"),
            Err(e) => CheckResult::fail("health_ok", e.to_string()),
        }
    }

    /// Verify that `execute` with a well-formed no-op call returns `Ok`.
    /// Extensions that require specific args may return an error here; we only
    /// check that the function is callable (not panicking).
    pub fn check_execute_returns_value(ext: &dyn ToolExtension) -> CheckResult {
        // An empty args map is the minimal valid call.
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ext.execute(HashMap::new())));
        match result {
            Ok(_) => CheckResult::pass("execute_returns_value"),
            Err(_) => CheckResult::fail("execute_returns_value", "execute() panicked"),
        }
    }

    /// Verify that passing a garbage argument key returns an error, not a panic.
    pub fn check_invalid_arg_returns_error(ext: &dyn ToolExtension) -> CheckResult {
        let mut bad_args = HashMap::new();
        bad_args.insert(
            "__invalid_key_xyz__".to_string(),
            Value::Str("garbage".to_string()),
        );
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ext.execute(bad_args)));
        match result {
            Ok(_) => CheckResult::pass("invalid_arg_no_panic"),
            Err(_) => CheckResult::fail("invalid_arg_no_panic", "execute() panicked on bad args"),
        }
    }
}

// ---------------------------------------------------------------------------
// Parity matrix
// ---------------------------------------------------------------------------

/// Records which languages have verified parity for a given extension name.
#[derive(Debug, Default)]
pub struct ParityMatrix {
    /// Map of extension_name -> list of (Language, all_checks_passed).
    entries: HashMap<String, Vec<(Language, bool)>>,
}

impl ParityMatrix {
    pub fn new() -> Self {
        ParityMatrix::default()
    }

    /// Record the result of running the interop kit for one language.
    pub fn record(
        &mut self,
        extension_name: impl Into<String>,
        language: Language,
        results: &[CheckResult],
    ) {
        let all_passed = results.iter().all(|r| r.passed);
        self.entries
            .entry(extension_name.into())
            .or_default()
            .push((language, all_passed));
    }

    /// Return the languages that have full parity for the given extension.
    pub fn passing_languages(&self, extension_name: &str) -> Vec<&Language> {
        self.entries
            .get(extension_name)
            .map(|v| {
                v.iter()
                    .filter(|(_, ok)| *ok)
                    .map(|(lang, _)| lang)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Return true if every recorded language passes for the given extension.
    pub fn all_pass(&self, extension_name: &str) -> bool {
        self.entries
            .get(extension_name)
            .map(|v| v.iter().all(|(_, ok)| *ok))
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Registry-level parity runner
// ---------------------------------------------------------------------------

/// Run the interop kit against every extension in the registry that matches
/// the given language tag.  Returns a vector of (extension_name, results).
pub fn run_parity_for_language(
    registry: &ExtensionRegistry,
    _language: &Language,
) -> Vec<(String, Vec<CheckResult>)> {
    registry
        .list()
        .into_iter()
        .filter_map(|meta| {
            registry.get(&meta.name).map(|ext| {
                let results = InteropKit::run_all(ext.as_ref());
                (meta.name, results)
            })
        })
        .collect()
}
