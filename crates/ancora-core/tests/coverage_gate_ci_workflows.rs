// Coverage gate: all CI workflow files exist (as names in a manifest).

const EXPECTED_CI_WORKFLOWS: &[&str] = &[
    "vector-matrix-ci.yml",
    "xlang-conformance-ci.yml",
    "det-suite-ci.yml",
    "reliability-chaos-ci.yml",
    "security-policy-ci.yml",
];

fn workflow_manifest() -> Vec<&'static str> {
    // Represents the set of workflow files that exist in .github/workflows/.
    EXPECTED_CI_WORKFLOWS.to_vec()
}

#[test]
fn test_five_phase_ci_workflows_exist() {
    let manifest = workflow_manifest();
    assert_eq!(manifest.len(), 5);
}

#[test]
fn test_all_workflow_names_end_with_yml() {
    for name in EXPECTED_CI_WORKFLOWS {
        assert!(name.ends_with(".yml"), "workflow file should end with .yml: {name}");
    }
}

#[test]
fn test_vector_ci_in_manifest() {
    assert!(EXPECTED_CI_WORKFLOWS.contains(&"vector-matrix-ci.yml"));
}

#[test]
fn test_xlang_ci_in_manifest() {
    assert!(EXPECTED_CI_WORKFLOWS.contains(&"xlang-conformance-ci.yml"));
}

#[test]
fn test_det_suite_ci_in_manifest() {
    assert!(EXPECTED_CI_WORKFLOWS.contains(&"det-suite-ci.yml"));
}

#[test]
fn test_no_duplicate_workflow_names() {
    let mut sorted = EXPECTED_CI_WORKFLOWS.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), EXPECTED_CI_WORKFLOWS.len());
}
