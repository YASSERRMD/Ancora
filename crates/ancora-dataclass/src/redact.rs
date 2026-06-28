use crate::label::SensitivityLevel;

pub struct RedactionConfig {
    pub redact_at_or_above: SensitivityLevel,
    pub mask: String,
}

impl RedactionConfig {
    pub fn new(redact_at_or_above: SensitivityLevel) -> Self {
        Self { redact_at_or_above, mask: "***REDACTED***".to_string() }
    }

    pub fn with_mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = mask.into();
        self
    }

    pub fn should_redact(&self, level: &SensitivityLevel) -> bool {
        level.is_at_least(&self.redact_at_or_above)
    }

    pub fn apply(&self, value: &str, level: &SensitivityLevel) -> String {
        if self.should_redact(level) {
            self.mask.clone()
        } else {
            value.to_string()
        }
    }
}
