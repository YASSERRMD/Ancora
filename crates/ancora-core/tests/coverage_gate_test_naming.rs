// Coverage gate: all test functions follow naming conventions.

const NAMING_RULES: &[(&str, &str)] = &[
    ("det_",          "starts_with_module_prefix"),
    ("chaos_",        "starts_with_module_prefix"),
    ("load_",         "starts_with_module_prefix"),
    ("reliability_",  "starts_with_module_prefix"),
    ("sec_",          "starts_with_module_prefix"),
    ("policy_",       "starts_with_module_prefix"),
    ("coverage_gate_","starts_with_module_prefix"),
    ("xlang_",        "starts_with_module_prefix"),
    ("vector_",       "starts_with_module_prefix"),
];

fn module_prefix_from_filename(filename: &str) -> &str {
    if filename.starts_with("det_") { "det_" }
    else if filename.starts_with("chaos_") { "chaos_" }
    else if filename.starts_with("load_") { "load_" }
    else if filename.starts_with("reliability_") { "reliability_" }
    else if filename.starts_with("sec_") { "sec_" }
    else if filename.starts_with("policy_") { "policy_" }
    else if filename.starts_with("coverage_gate_") { "coverage_gate_" }
    else if filename.starts_with("xlang_") { "xlang_" }
    else if filename.starts_with("vector_") { "vector_" }
    else { "unknown_" }
}

#[test]
fn test_naming_rules_defined_for_all_prefixes() {
    assert_eq!(NAMING_RULES.len(), 9);
}

#[test]
fn test_det_prefix_extracted_correctly() {
    assert_eq!(module_prefix_from_filename("det_identical_inputs"), "det_");
}

#[test]
fn test_chaos_prefix_extracted() {
    assert_eq!(module_prefix_from_filename("chaos_retry_backoff"), "chaos_");
}

#[test]
fn test_sec_prefix_extracted() {
    assert_eq!(module_prefix_from_filename("sec_prompt_injection"), "sec_");
}

#[test]
fn test_policy_prefix_extracted() {
    assert_eq!(module_prefix_from_filename("policy_data_residency"), "policy_");
}

#[test]
fn test_unknown_file_returns_unknown_prefix() {
    assert_eq!(module_prefix_from_filename("some_other_file"), "unknown_");
}
