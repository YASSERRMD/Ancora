// Coverage gate: all required policy properties have at least one test.

const POLICY_PROPERTIES: &[&str] = &[
    "data_residency",
    "model_allowlist",
    "cost_ceiling",
    "data_retention",
    "data_sovereignty",
    "consent_gate",
    "gdpr_erasure",
    "offline_mode",
];

const POLICY_TEST_MAP: &[(&str, &str)] = &[
    ("data_residency",   "policy_data_residency"),
    ("model_allowlist",  "policy_model_allowlist"),
    ("cost_ceiling",     "policy_cost_ceiling"),
    ("data_retention",   "policy_retention"),
    ("data_sovereignty", "policy_sovereignty"),
    ("consent_gate",     "policy_consent"),
    ("gdpr_erasure",     "policy_gdpr_erasure"),
    ("offline_mode",     "policy_offline_mode"),
];

#[test]
fn test_all_policy_properties_covered() {
    let covered: Vec<&str> = POLICY_TEST_MAP.iter().map(|(p, _)| *p).collect();
    for prop in POLICY_PROPERTIES {
        assert!(covered.contains(prop), "no test for policy property: {prop}");
    }
}

#[test]
fn test_policy_properties_count_is_8() {
    assert_eq!(POLICY_PROPERTIES.len(), 8);
}

#[test]
fn test_all_test_names_start_with_policy() {
    for (_, test) in POLICY_TEST_MAP {
        assert!(test.starts_with("policy_"), "policy test should start with policy_: {test}");
    }
}

#[test]
fn test_no_duplicate_policy_properties() {
    let mut sorted = POLICY_PROPERTIES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), POLICY_PROPERTIES.len());
}

#[test]
fn test_gdpr_erasure_maps_to_correct_test() {
    let entry = POLICY_TEST_MAP.iter().find(|(p, _)| *p == "gdpr_erasure");
    assert_eq!(entry.map(|(_, t)| *t), Some("policy_gdpr_erasure"));
}
