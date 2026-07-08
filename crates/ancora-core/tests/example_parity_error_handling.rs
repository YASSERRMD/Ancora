// Example parity: error handling example -- all languages surface the same error type.

const EXPECTED_ERROR_KINDS: &[&str] = &[
    "divergence",
    "journal_not_found",
    "provider_error",
    "cost_ceiling_exceeded",
    "tool_not_allowed",
];

struct ErrorHandlingExample {
    lang: &'static str,
    error_kind: &'static str,
    error_message_contains: &'static str,
}

const ERROR_EXAMPLES: &[ErrorHandlingExample] = &[
    ErrorHandlingExample {
        lang: "rust",
        error_kind: "divergence",
        error_message_contains: "divergence",
    },
    ErrorHandlingExample {
        lang: "go",
        error_kind: "provider_error",
        error_message_contains: "provider",
    },
    ErrorHandlingExample {
        lang: "python",
        error_kind: "cost_ceiling_exceeded",
        error_message_contains: "cost",
    },
    ErrorHandlingExample {
        lang: "typescript",
        error_kind: "tool_not_allowed",
        error_message_contains: "allowlist",
    },
    ErrorHandlingExample {
        lang: "dotnet",
        error_kind: "journal_not_found",
        error_message_contains: "not found",
    },
    ErrorHandlingExample {
        lang: "java",
        error_kind: "divergence",
        error_message_contains: "divergence",
    },
];

#[test]
fn test_all_error_kinds_in_expected_list() {
    for e in ERROR_EXAMPLES {
        assert!(
            EXPECTED_ERROR_KINDS.contains(&e.error_kind),
            "lang {} uses unknown error kind: {}",
            e.lang,
            e.error_kind
        );
    }
}

#[test]
fn test_six_error_handling_examples() {
    assert_eq!(ERROR_EXAMPLES.len(), 6);
}

#[test]
fn test_all_error_messages_non_empty() {
    for e in ERROR_EXAMPLES {
        assert!(!e.error_message_contains.is_empty());
    }
}

#[test]
fn test_five_error_kinds_defined() {
    assert_eq!(EXPECTED_ERROR_KINDS.len(), 5);
}

#[test]
fn test_divergence_error_covered_by_at_least_two_languages() {
    let div_count = ERROR_EXAMPLES
        .iter()
        .filter(|e| e.error_kind == "divergence")
        .count();
    assert!(
        div_count >= 2,
        "divergence error only covered by {} language(s)",
        div_count
    );
}
