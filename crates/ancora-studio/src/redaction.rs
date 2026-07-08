//! Redaction - ensures sensitive fields are hidden before rendering in the UI.

#[derive(Debug, Clone, PartialEq)]
pub enum RedactionPolicy {
    /// Never show the field.
    AlwaysRedact,
    /// Show if the viewer has the given role.
    RequireRole(String),
    /// Show always.
    NeverRedact,
}

#[derive(Debug, Clone)]
pub struct FieldRedactionRule {
    pub field_path: String,
    pub policy: RedactionPolicy,
    pub replacement: String,
}

impl FieldRedactionRule {
    pub fn new(field_path: impl Into<String>, policy: RedactionPolicy) -> Self {
        Self {
            field_path: field_path.into(),
            policy,
            replacement: "[REDACTED]".into(),
        }
    }

    pub fn with_replacement(mut self, replacement: impl Into<String>) -> Self {
        self.replacement = replacement.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct ViewerContext {
    pub roles: Vec<String>,
}

impl ViewerContext {
    pub fn new(roles: Vec<String>) -> Self {
        Self { roles }
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}

pub struct RedactionEngine {
    rules: Vec<FieldRedactionRule>,
}

impl RedactionEngine {
    pub fn new(rules: Vec<FieldRedactionRule>) -> Self {
        Self { rules }
    }

    pub fn should_redact(&self, field_path: &str, viewer: &ViewerContext) -> bool {
        for rule in &self.rules {
            if rule.field_path == field_path {
                return match &rule.policy {
                    RedactionPolicy::AlwaysRedact => true,
                    RedactionPolicy::NeverRedact => false,
                    RedactionPolicy::RequireRole(role) => !viewer.has_role(role),
                };
            }
        }
        false
    }

    pub fn apply(&self, field_path: &str, value: &str, viewer: &ViewerContext) -> String {
        if self.should_redact(field_path, viewer) {
            self.replacement_for(field_path)
        } else {
            value.to_string()
        }
    }

    fn replacement_for(&self, field_path: &str) -> String {
        self.rules
            .iter()
            .find(|r| r.field_path == field_path)
            .map(|r| r.replacement.clone())
            .unwrap_or_else(|| "[REDACTED]".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> RedactionEngine {
        RedactionEngine::new(vec![
            FieldRedactionRule::new("prompt", RedactionPolicy::AlwaysRedact),
            FieldRedactionRule::new("response", RedactionPolicy::RequireRole("admin".into())),
            FieldRedactionRule::new("label", RedactionPolicy::NeverRedact),
        ])
    }

    #[test]
    fn test_always_redact() {
        let eng = engine();
        let viewer = ViewerContext::new(vec!["admin".into()]);
        assert!(eng.should_redact("prompt", &viewer));
    }

    #[test]
    fn test_role_redact_non_admin() {
        let eng = engine();
        let viewer = ViewerContext::new(vec!["user".into()]);
        assert!(eng.should_redact("response", &viewer));
    }

    #[test]
    fn test_role_redact_admin_pass() {
        let eng = engine();
        let viewer = ViewerContext::new(vec!["admin".into()]);
        assert!(!eng.should_redact("response", &viewer));
    }

    #[test]
    fn test_never_redact() {
        let eng = engine();
        let viewer = ViewerContext::new(vec![]);
        assert!(!eng.should_redact("label", &viewer));
    }

    #[test]
    fn test_apply() {
        let eng = engine();
        let viewer = ViewerContext::new(vec![]);
        let result = eng.apply("prompt", "hello", &viewer);
        assert_eq!(result, "[REDACTED]");
        let result2 = eng.apply("label", "my run", &viewer);
        assert_eq!(result2, "my run");
    }
}
