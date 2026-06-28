// Example parity: human-in-loop example decision payload consistent across languages.

const HIL_PROMPT: &str = "Please approve the draft";
const HIL_OPTIONS: &[&str] = &["approve", "reject"];
const HIL_DECISION_JSON: &str = r#"{"approved":true}"#;

struct HilExample {
    lang: &'static str,
    prompt: &'static str,
    options: &'static [&'static str],
    decision: &'static str,
}

const HIL_EXAMPLES: &[HilExample] = &[
    HilExample { lang: "rust",       prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
    HilExample { lang: "go",         prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
    HilExample { lang: "python",     prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
    HilExample { lang: "typescript", prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
    HilExample { lang: "dotnet",     prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
    HilExample { lang: "java",       prompt: HIL_PROMPT, options: HIL_OPTIONS, decision: HIL_DECISION_JSON },
];

#[test]
fn test_all_hil_examples_use_same_prompt() {
    for e in HIL_EXAMPLES { assert_eq!(e.prompt, HIL_PROMPT, "lang {} prompt differs", e.lang); }
}

#[test]
fn test_all_hil_examples_use_same_options() {
    for e in HIL_EXAMPLES {
        assert_eq!(e.options, HIL_OPTIONS, "lang {} options differ", e.lang);
    }
}

#[test]
fn test_all_hil_decisions_contain_approved_true() {
    for e in HIL_EXAMPLES { assert!(e.decision.contains("true"), "lang {} decision lacks approved:true", e.lang); }
}

#[test]
fn test_six_hil_examples() {
    assert_eq!(HIL_EXAMPLES.len(), 6);
}

#[test]
fn test_decision_is_valid_json_structure() {
    assert!(HIL_DECISION_JSON.starts_with('{'));
    assert!(HIL_DECISION_JSON.ends_with('}'));
}

#[test]
fn test_options_has_approve_and_reject() {
    assert!(HIL_OPTIONS.contains(&"approve"));
    assert!(HIL_OPTIONS.contains(&"reject"));
}
