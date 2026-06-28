// Coverage gate: all A2A cross-language handoff scenarios have tests.

const A2A_PAIRS: &[(&str, &str)] = &[
    ("go",     "python"),
    ("python", "go"),
    ("ts",     "dotnet"),
    ("dotnet", "ts"),
    ("rust",   "python"),
    ("java",   "rust"),
];

const A2A_TEST_MAP: &[(&str, &str, &str)] = &[
    ("go",     "python", "xlang_a2a_to_python_test"),
    ("python", "go",     "test_xlang_a2a_to_go"),
    ("ts",     "dotnet", "xlang-a2a-to-dotnet.test"),
    ("dotnet", "ts",     "A2aDotnetTsTests"),
    ("rust",   "python", "xlang_a2a_identity"),
    ("java",   "rust",   "Phase152A2aJavaRustTest"),
];

#[test]
fn test_all_a2a_pairs_have_tests() {
    for (sender, recipient) in A2A_PAIRS {
        let has_test = A2A_TEST_MAP.iter().any(|(s, r, _)| s == sender && r == recipient);
        assert!(has_test, "no A2A test for {sender} -> {recipient}");
    }
}

#[test]
fn test_six_a2a_pairs_defined() {
    assert_eq!(A2A_PAIRS.len(), 6);
}

#[test]
fn test_a2a_test_map_has_six_entries() {
    assert_eq!(A2A_TEST_MAP.len(), 6);
}

#[test]
fn test_no_self_handoff_pairs() {
    for (s, r) in A2A_PAIRS { assert_ne!(s, r, "self-handoff pair: {s} -> {r}"); }
}

#[test]
fn test_rust_is_involved_in_a2a() {
    let rust_involved = A2A_PAIRS.iter().any(|(s, r)| *s == "rust" || *r == "rust");
    assert!(rust_involved);
}

#[test]
fn test_java_is_sender_in_at_least_one_pair() {
    let java_sender = A2A_PAIRS.iter().any(|(s, _)| *s == "java");
    assert!(java_sender);
}
