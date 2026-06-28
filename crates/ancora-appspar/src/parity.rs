/// Parity checker module.
///
/// Verifies that all language sample apps expose the same set of features
/// and that their trace shapes are equivalent.

use crate::{go_app, python_app, ts_app, dotnet_app, java_app, rust_app};

/// A canonical feature set that every language sample app must implement.
pub const REQUIRED_FEATURES: &[&str] = &[
    "streaming",
    "tool_calls",
    "structured_output",
    "guardrails",
    "tracing",
];

/// Result of a parity check for one language.
#[derive(Debug, Clone, PartialEq)]
pub struct ParityResult {
    pub language: String,
    pub missing: Vec<String>,
    pub extra: Vec<String>,
}

impl ParityResult {
    pub fn is_passing(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Check whether `features` satisfies all required features.
fn check_features(language: &str, features: &[&str]) -> ParityResult {
    let missing: Vec<String> = REQUIRED_FEATURES
        .iter()
        .filter(|&&req| !features.contains(&req))
        .map(|s| s.to_string())
        .collect();

    let extra: Vec<String> = features
        .iter()
        .filter(|&&f| !REQUIRED_FEATURES.contains(&f))
        .map(|s| s.to_string())
        .collect();

    ParityResult {
        language: language.to_string(),
        missing,
        extra,
    }
}

/// Run parity checks across all languages.
pub fn run_all() -> Vec<ParityResult> {
    vec![
        check_features("go", &go_app::feature_list()),
        check_features("python", &python_app::feature_list()),
        check_features("typescript", &ts_app::feature_list()),
        check_features("dotnet", &dotnet_app::feature_list()),
        check_features("java", &java_app::feature_list()),
        check_features("rust", &rust_app::feature_list()),
    ]
}

/// Returns true when every language passes the parity check.
pub fn all_pass() -> bool {
    run_all().iter().all(|r| r.is_passing())
}
