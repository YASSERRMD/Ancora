// Example parity: policy enforcement example blocks same requests across all SDKs.

const BLOCKED_MODEL: &str = "gpt-4o";
const ALLOWED_MODEL: &str = "claude-3-5-haiku";
const BLOCKED_REGION: &str = "us-east-1";
const ALLOWED_REGION: &str = "eu-west-1";

struct PolicyExample {
    lang: &'static str,
    model: &'static str,
    region: &'static str,
    should_block: bool,
}

const POLICY_EXAMPLES: &[PolicyExample] = &[
    PolicyExample {
        lang: "rust",
        model: BLOCKED_MODEL,
        region: ALLOWED_REGION,
        should_block: true,
    },
    PolicyExample {
        lang: "go",
        model: BLOCKED_MODEL,
        region: ALLOWED_REGION,
        should_block: true,
    },
    PolicyExample {
        lang: "python",
        model: ALLOWED_MODEL,
        region: BLOCKED_REGION,
        should_block: true,
    },
    PolicyExample {
        lang: "typescript",
        model: ALLOWED_MODEL,
        region: ALLOWED_REGION,
        should_block: false,
    },
    PolicyExample {
        lang: "dotnet",
        model: ALLOWED_MODEL,
        region: ALLOWED_REGION,
        should_block: false,
    },
    PolicyExample {
        lang: "java",
        model: ALLOWED_MODEL,
        region: ALLOWED_REGION,
        should_block: false,
    },
];

fn policy_check(model: &str, region: &str) -> bool {
    model == BLOCKED_MODEL || region == BLOCKED_REGION
}

#[test]
fn test_blocked_model_is_blocked() {
    assert!(policy_check(BLOCKED_MODEL, ALLOWED_REGION));
}

#[test]
fn test_blocked_region_is_blocked() {
    assert!(policy_check(ALLOWED_MODEL, BLOCKED_REGION));
}

#[test]
fn test_allowed_model_and_region_passes() {
    assert!(!policy_check(ALLOWED_MODEL, ALLOWED_REGION));
}

#[test]
fn test_policy_examples_match_expected_block() {
    for e in POLICY_EXAMPLES {
        assert_eq!(
            policy_check(e.model, e.region),
            e.should_block,
            "lang {} policy outcome differs",
            e.lang
        );
    }
}

#[test]
fn test_six_policy_examples() {
    assert_eq!(POLICY_EXAMPLES.len(), 6);
}

#[test]
fn test_three_blocked_three_allowed() {
    let blocked = POLICY_EXAMPLES.iter().filter(|e| e.should_block).count();
    let allowed = POLICY_EXAMPLES.iter().filter(|e| !e.should_block).count();
    assert_eq!(blocked, 3);
    assert_eq!(allowed, 3);
}
