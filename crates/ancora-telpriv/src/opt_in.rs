/// Opt-in controls for sensitive telemetry capture.
///
/// Certain telemetry (prompt text, completion text, user identifiers) is
/// never captured unless the operator has explicitly opted in. This module
/// provides the registry of opt-in flags and a guard that checks them.

/// The set of opt-in features available.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptInFeature {
    /// Capture the raw prompt text in spans.
    PromptCapture,
    /// Capture the raw completion text in spans.
    CompletionCapture,
    /// Include hashed user identifiers in telemetry.
    UserIdCorrelation,
    /// Export eval prompt/completion data to the telemetry sink.
    EvalTextExport,
    /// Send full stack traces (may include variable values).
    FullStackTraces,
}

/// Registry of operator opt-ins.
#[derive(Debug, Clone, Default)]
pub struct OptInRegistry {
    enabled: Vec<OptInFeature>,
}

impl OptInRegistry {
    /// Create an empty registry (all features disabled).
    pub fn new() -> Self {
        OptInRegistry {
            enabled: Vec::new(),
        }
    }

    /// Enable a feature. Idempotent.
    pub fn enable(&mut self, feature: OptInFeature) {
        if !self.enabled.contains(&feature) {
            self.enabled.push(feature);
        }
    }

    /// Disable a feature if it was enabled.
    pub fn disable(&mut self, feature: &OptInFeature) {
        self.enabled.retain(|f| f != feature);
    }

    /// Check whether a feature is enabled.
    pub fn is_enabled(&self, feature: &OptInFeature) -> bool {
        self.enabled.contains(feature)
    }

    /// Construct from an environment variable string.
    /// Format: comma-separated feature names.
    pub fn from_env_str(s: &str) -> Self {
        let mut reg = Self::new();
        for token in s.split(',') {
            match token.trim().to_ascii_lowercase().as_str() {
                "prompt_capture" => reg.enable(OptInFeature::PromptCapture),
                "completion_capture" => reg.enable(OptInFeature::CompletionCapture),
                "user_id_correlation" => reg.enable(OptInFeature::UserIdCorrelation),
                "eval_text_export" => reg.enable(OptInFeature::EvalTextExport),
                "full_stack_traces" => reg.enable(OptInFeature::FullStackTraces),
                _ => {} // unknown tokens are silently ignored
            }
        }
        reg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_capture_off_by_default() {
        let reg = OptInRegistry::new();
        assert!(!reg.is_enabled(&OptInFeature::PromptCapture));
    }

    #[test]
    fn enable_and_check() {
        let mut reg = OptInRegistry::new();
        reg.enable(OptInFeature::PromptCapture);
        assert!(reg.is_enabled(&OptInFeature::PromptCapture));
    }

    #[test]
    fn disable_removes_feature() {
        let mut reg = OptInRegistry::new();
        reg.enable(OptInFeature::CompletionCapture);
        reg.disable(&OptInFeature::CompletionCapture);
        assert!(!reg.is_enabled(&OptInFeature::CompletionCapture));
    }

    #[test]
    fn from_env_str_parses() {
        let reg = OptInRegistry::from_env_str("prompt_capture, eval_text_export");
        assert!(reg.is_enabled(&OptInFeature::PromptCapture));
        assert!(reg.is_enabled(&OptInFeature::EvalTextExport));
        assert!(!reg.is_enabled(&OptInFeature::CompletionCapture));
    }
}
