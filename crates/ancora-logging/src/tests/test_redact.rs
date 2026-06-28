#[cfg(test)]
mod tests {
    use crate::redact::{is_clean, redact_json};

    #[test]
    fn secrets_redacted_in_json() {
        let json = r#"{"api_key":"super-secret","message":"hello"}"#;
        let clean = redact_json(json);
        assert!(clean.contains("[REDACTED]"));
        assert!(!clean.contains("super-secret"));
    }

    #[test]
    fn non_secret_fields_preserved() {
        let json = r#"{"message":"hello","level":"Info"}"#;
        let clean = redact_json(json);
        assert!(clean.contains("hello"));
        assert!(clean.contains("Info"));
    }

    #[test]
    fn is_clean_returns_false_for_exposed_secret() {
        let json = r#"{"token":"my-token"}"#;
        assert!(!is_clean(json));
    }

    #[test]
    fn is_clean_returns_true_when_redacted() {
        let json = r#"{"token":"[REDACTED]"}"#;
        assert!(is_clean(json));
    }
}
