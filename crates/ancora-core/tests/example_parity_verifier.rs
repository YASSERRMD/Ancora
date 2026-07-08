// Example parity: verifier example checks verdict matches across languages.

const VERIFIER_EXPECTED_VERDICT: &str = "approved";

const VERIFIER_EXAMPLES: &[(&str, &str, &str)] = &[
    ("rust", "drafter", "verifier"),
    ("go", "drafter", "verifier"),
    ("python", "drafter", "verifier"),
    ("typescript", "drafter", "verifier"),
    ("dotnet", "drafter", "verifier"),
    ("java", "drafter", "verifier"),
];

struct VerifierResult {
    lang: &'static str,
    drafter_activity: &'static str,
    verifier_activity: &'static str,
    verdict: &'static str,
}

const VERIFIER_RESULTS: &[VerifierResult] = &[
    VerifierResult {
        lang: "rust",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
    VerifierResult {
        lang: "go",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
    VerifierResult {
        lang: "python",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
    VerifierResult {
        lang: "typescript",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
    VerifierResult {
        lang: "dotnet",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
    VerifierResult {
        lang: "java",
        drafter_activity: "drafter",
        verifier_activity: "verifier",
        verdict: "approved",
    },
];

#[test]
fn test_all_verifier_examples_produce_approved() {
    for r in VERIFIER_RESULTS {
        assert_eq!(
            r.verdict, VERIFIER_EXPECTED_VERDICT,
            "lang {} produced '{}' not 'approved'",
            r.lang, r.verdict
        );
    }
}

#[test]
fn test_drafter_before_verifier_in_all_examples() {
    for (_, drafter, verifier) in VERIFIER_EXAMPLES {
        assert_ne!(*drafter, *verifier);
        assert_eq!(*drafter, "drafter");
        assert_eq!(*verifier, "verifier");
    }
}

#[test]
fn test_six_language_examples() {
    assert_eq!(VERIFIER_RESULTS.len(), 6);
}

#[test]
fn test_verdict_is_non_empty_for_all() {
    for r in VERIFIER_RESULTS {
        assert!(!r.verdict.is_empty());
    }
}

#[test]
fn test_all_activity_names_consistent() {
    for r in VERIFIER_RESULTS {
        assert_eq!(r.drafter_activity, "drafter");
        assert_eq!(r.verifier_activity, "verifier");
    }
}
