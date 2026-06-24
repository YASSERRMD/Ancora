use crate::error::PolicyError;
use crate::policy::Policy;

/// Callback hook invoked before writing to the journal to redact PII.
pub trait PiiRedactor: Send + Sync {
    /// Return a copy of `text` with any PII redacted.
    fn redact(&self, text: &str) -> String;
}

/// Apply PII redaction before a journal write if the policy requires it.
pub fn redact_if_required(
    policy: &Policy,
    redactor: Option<&dyn PiiRedactor>,
    text: &str,
) -> Result<String, PolicyError> {
    if policy.require_pii_redaction {
        let r = redactor.ok_or_else(|| PolicyError::PiiDetected("no redactor configured".into()))?;
        return Ok(r.redact(text));
    }
    Ok(text.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StarRedactor;
    impl PiiRedactor for StarRedactor {
        fn redact(&self, text: &str) -> String {
            text.replace("secret", "***")
        }
    }

    #[test]
    fn pii_is_redacted_in_journal_when_policy_demands() {
        let mut policy = Policy::new();
        policy.require_pii_redaction = true;
        let redactor = StarRedactor;
        let result = redact_if_required(&policy, Some(&redactor), "the secret is here").unwrap();
        assert_eq!(result, "the *** is here");
    }

    #[test]
    fn no_redaction_when_policy_does_not_require_it() {
        let policy = Policy::new();
        let result = redact_if_required(&policy, None, "the secret is here").unwrap();
        assert_eq!(result, "the secret is here");
    }
}
