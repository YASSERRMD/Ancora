//! Interoperability test kit for Ancora plugins.
//!
//! Provides helper types and assertions that plugin authors can use
//! to verify compatibility with the Ancora runtime contract.

/// Result type for interop checks.
pub type InteropResult = Result<(), InteropError>;

/// An error produced by an interop check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteropError {
    pub check: &'static str,
    pub message: String,
}

impl std::fmt::Display for InteropError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.check, self.message)
    }
}

/// A single interop check.
pub struct InteropCheck {
    pub name: &'static str,
    pub description: &'static str,
    pub run: fn() -> InteropResult,
}

/// A suite of interop checks.
pub struct InteropSuite {
    checks: Vec<InteropCheck>,
}

impl InteropSuite {
    /// Create a new empty suite.
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    /// Add a check to the suite.
    pub fn add_check(mut self, check: InteropCheck) -> Self {
        self.checks.push(check);
        self
    }

    /// Run all checks and return a list of failures.
    pub fn run_all(&self) -> Vec<InteropError> {
        self.checks.iter().filter_map(|c| (c.run)().err()).collect()
    }

    /// Returns `true` if all checks pass.
    pub fn all_pass(&self) -> bool {
        self.run_all().is_empty()
    }
}

impl Default for InteropSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard check: verify that the plugin name is non-empty.
pub fn check_plugin_name_not_empty(name: &str) -> InteropResult {
    if name.is_empty() {
        return Err(InteropError {
            check: "plugin-name",
            message: "plugin name must not be empty".into(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_fails_check() {
        assert!(check_plugin_name_not_empty("").is_err());
    }

    #[test]
    fn non_empty_name_passes_check() {
        assert!(check_plugin_name_not_empty("my-plugin").is_ok());
    }

    #[test]
    fn suite_with_passing_checks() {
        let suite = InteropSuite::new().add_check(InteropCheck {
            name: "always-pass",
            description: "Always passes",
            run: || Ok(()),
        });
        assert!(suite.all_pass());
    }

    #[test]
    fn suite_with_failing_check() {
        let suite = InteropSuite::new().add_check(InteropCheck {
            name: "always-fail",
            description: "Always fails",
            run: || {
                Err(InteropError {
                    check: "always-fail",
                    message: "intentional failure".into(),
                })
            },
        });
        assert!(!suite.all_pass());
        assert_eq!(suite.run_all().len(), 1);
    }
}
